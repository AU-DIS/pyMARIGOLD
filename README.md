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
