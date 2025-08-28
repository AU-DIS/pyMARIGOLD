# pyMARIGOLD
A python package for MARIGOLD kmeans clustering

## Installation
The latest version can be installed directly with:

```shell
pip install git+https://github.com/AU-DIS/pyMARIGOLD
```

## Quickstart

Some documentation can be found in marigold.py.

This example shows how to load and run marigold:

```python
import marigold as mg
import numpy as np

dataset = np.ones((5, 10), dtype=np.double)
print(dataset)

result = mg.marigold(X=dataset, n_clusters=2, init="first")

print(result)
```

Alternatively, run multiple times and get the best one:
```python
import marigold as mg
import numpy as np

dataset = np.ones((5, 10), dtype=np.double)
print(dataset)

result = mg.marigold(X=dataset, n_clusters=2, n_init=10)

print(result)
```

## Data Transformation pre-step
Note that this version does not include the DCT pre-transformation of data. Hence faster results may be achieved if pre-processing is feasible. The following results show how to DCT transform data in python:
```python
from scipy.fftpack import dct
from keras.datasets import mnist
import matplotlib.pyplot as plt

# Obtain some data
(digits, labels), (digits_test,labels_test) = mnist.load_data()

plt.gray()
plt.matshow(digits[303])
plt.show()

# Transform in both dimension for best results
dct_digit1 = dct(digits[302], norm='ortho')
dct_digit1 = dct(dct_digit1, norm='ortho', axis=0)

plt.gray()
plt.matshow(dct_digit2.reshape((28,28)))
plt.show()

# Transformation does not change the clustering. Euclidian distances stay the same.
dct_digit2 = dct(digits[303], norm='ortho')
dct_digit2 = dct(dct_digit2, norm='ortho', axis=0)

real_dist = euclidean_distances(digits[302].flatten().reshape(1,-1), digits[303].flatten().reshape(1,-1))
dct_dist = euclidean_distances(dct_digit1.flatten().reshape(1,-1), dct_digit2.flatten().reshape(1,-1))

print(real_dist)
print(dct_dist)
```

> [!NOTE]
> * You do not have to run the transformation every time you run Marigold. Simply save your transformed data once, and use it for all future K-means runs.
> * Marigold works without the the data transformation, and is still likely to perform well by relying on Elkan pruning.
  
## Multithreading
Currently this package does not support multithreading, hence scipy may be faster if multiple cores are available. This section will be updated if support is added.  

## Modify
The wrapper is dependent on the pre-compiled .so, .dll files.
The cpp files for theese are included and may be modified. To recompile the shared library files, the following commands can be used:

Build for Linux
```shell
g++ -fPIC -shared -O3 -o marigold.so runner.cpp
```
Build for Windows 64bit
```shell
x86_64-w64-mingw32-g++-win32 -fPIC -static -shared -mwindows -O3 -o marigold.dll runner.cpp -D BUILD_LIB
```
