from __future__ import annotations

import ctypes
import os
import pathlib
import platform
import warnings
from typing import Callable
from typing import Literal
from typing import Union

import numpy as np
import numpy.typing as npt
from numpy.ctypeslib import ndpointer
from numpy.random import RandomState

Init_t = Union[
    npt.NDArray[np.double],
    Callable[..., npt.NDArray[np.double]],
    Literal["random", "k-means++", "first"],
]


def marigold(
    X: npt.NDArray[np.double],
    n_clusters: int = 8,
    init: Init_t = "random",
    n_init: int | Literal["auto"] = "auto",
) -> tuple[npt.NDArray[np.double], npt.NDArray[np.int64], float, int]:
    est = MARIGOLD(X=X, n_clusters=n_clusters, init=init, n_init=n_init).fit()
    return est.final_centroids, est.labels_, est.final_inertia, est.final_iter


class MARIGOLD:
    """K-Means clustering with the MARIGOLD algorithm.


    Parameters
    ----------

    dataset : numpy arraylike
        The dataset will be passed to C++ as a ND pointer and stored there.
        This avoids copying or loading of the data during the pass.
        Highly experimental as this is the first time I have done something
        like this.
        If Segmentation faults are observed, this handling is a likely
        candidate to where things went wrong.
        Note that the C++ is oblivious to the actual size of the dataset
        and blindly trust the provided dimensions n and d.
        If they are set wrong, it will access unintended memory.

    n : int
        The number of points in the dataset.
        Set automatically using the marigold function.
        

    d : int
        The number of dimensions of each point in the dataset.
        Set automatically using the marigold function.

    k : int
        The number of clusters to form
    
    init : numpy array, "random", "k-means++", "first"
        The initial clusters. Can be set to a numpy array for custom initialization.
        "random" - set clusters to random datapoints
        "k-means" - not implemented. defaults to "random"
        "first" - set clusters to the first k datapoints
        
    n_init : int, "auto"
        Run clustering n_init times and return the results with lowest inertia.
        "auto" - defaults to 1.
        If n_init is not "auto", it will set init to "random" overwriting user defined values.


    Notes
    -----
    This is an evolving wrapper for a c++ implementation of MARIGOLD.
    Currently some variables and settings are not exposed.The goal is to
    provide the same flexibility as sklearn kmeans.

    Important missing features:
    Max_iter.
    Max iter is currently unexposed and fixed to 1000 iterations.

    Verbose mode.
    It prints what it prints, and you can't stop it.
    
    Warnings.
    Correctness of provided values for dataset and its size is currently not checked. Expect unspecified errors if manually set incorrectly .

    """

    def __init__(
        self,
        X: npt.NDArray[np.double],
        n_clusters: int = 8,
        init: Init_t = "random",
        n_init: int | Literal["auto"] = "auto",
    ) -> None:
        self.dataset = X
        # TODO: Check dtype is double
        self.init: Init_t = init
        self.n = X.shape[0]
        self.d = X.shape[1]
        self.n_clusters = n_clusters
        self.random_state = RandomState()
        # self._init_centroids(init)
        self.n_init: int = self._solve_n_init(n_init)

    def _solve_n_init(self, n_init: int | Literal["auto"]) -> int:
        # This may be extended in the future
        #  to allow more case specific standard initializations
        if n_init == "auto":
            return 1
        # overwrite init to random
        self.init = "random"
        return n_init

    def _init_centroids(self, init: Init_t) -> None:
        if isinstance(init, str) and init == "k-means++":
            warnings.warn(
                "K-means++ is not implemented for MARIGOLD.\
                    Continuing with random initialization",
            )
            init = "random"
        elif callable(init):
            warnings.warn(
                "Callable initialization is not implemented for MARIGOLD.\
                      Continuing with random initialization",
            )
            init = "random"
        elif isinstance(init, np.ndarray):
            # TODO: check that sizes are correct
            if not init.shape[0] == self.n_clusters:
                warnings.warn(
                    f"Inital center data contains {init.shape[0]} points,\
                          but n_clusters is set to {self.n_clusters}",
                )
            if not init.shape[1] == self.d:
                raise ValueError(
                    f"Initial features {init.shape[1]} does not align \
                          with data features {self.d}",
                )
            self.centroids = init
        elif isinstance(init, str) and init == "random":
            chosen = self.random_state.permutation(self.n)[: self.n_clusters]
            self.centroids = np.copy(self.dataset[chosen])
        elif isinstance(init, str) and init == "first":
            self.centroids = np.copy(self.dataset[: self.n_clusters])

    def fit(self) -> MARIGOLD:
        libname = pathlib.Path(__file__).parent.absolute() / "src"
        c_lib = None
        print(libname)
        # TODO: This should not be in fit but solved at init.
        if platform.system() == "Linux":
            for f_name in os.listdir(libname):
                if f_name.endswith(".so"):

                    c_lib = ctypes.CDLL(str(libname) + "/" + f_name)
                    break
        elif platform.system() == "Windows":
            for f_name in os.listdir(libname):
                if f_name.endswith(".dll"):
                    c_lib = ctypes.CDLL(str(libname) + "/" + f_name)
                    break

        if c_lib is None:
            raise FileNotFoundError(c_lib)

        run = getattr(c_lib, "run")

        self.final_inertia = float("inf")

        res: list[int]
        for _ in range(self.n_init):
            self._init_centroids(self.init)
            out_centroids = (ctypes.c_double * (self.n_clusters * self.d))()

            # ndpointer(ctypes.c_double, flags="C_CONTIGUOUS")()
            # self.final_centroids = np.zeros((self.k,self.d),dtype=np.double)
            final_iter = ctypes.c_int(0)
            final_inertia = ctypes.c_double(0)
            run.restype = ctypes.POINTER(ctypes.c_int)
            run.argtypes = [
                ndpointer(ctypes.c_double, flags="C_CONTIGUOUS"),
                ctypes.c_int,
                ctypes.c_int,
                ctypes.c_int,
                ndpointer(ctypes.c_double, flags="C_CONTIGUOUS"),
                ctypes.POINTER(ctypes.c_double),
                ctypes.POINTER(ctypes.c_int),
            ]

            res = run(
                self.dataset,
                self.n,
                self.d,
                self.n_clusters,
                self.centroids,
                ctypes.cast(out_centroids, ctypes.POINTER(ctypes.c_double)),
                ctypes.byref(final_iter),
                ctypes.byref(final_inertia),
            )

            l_: list[int] = [res[i] for i in range(self.n)]

            # centroids_ptr = centroids_ptr.value
            c_: list[list[float]] = [
                [out_centroids[i * self.d + j] for j in range(self.d)]
                for i in range(self.n_clusters)
            ]
            
            getattr(c_lib, "clear")

            # If inertia is better the save results.
            if final_inertia.value < self.final_inertia:
                self.final_iter = final_iter.value
                self.final_inertia = final_inertia.value
                self.final_centroids = np.array(c_)
                self.labels_ = np.array(l_)

        return self


if __name__ == "__main__":
    # dataset = np.ones((5, 10), dtype=np.double)
    dataset = np.random.rand(100, 10)
    # print(dataset)
    result = marigold(dataset, 2, n_init=10)
    print(result)
