# pyMARIGOLD
A python package for MARIGOLD kmeans clustering

## Installation
The latest version can be installed directly with:

```shell
pip install git+https://github.com/AU-DIS/pyMARIGOLD
```

## Quickstart

This example shows how to load and run marigold:

```python
import marigold as mg 
import numpy as np

dataset = np.ones((5, 10), dtype=np.double)
print(dataset)

result = mg.marigold(X=dataset, n_clusters=2, init="first")

print(result)
```
