#!/usr/bin/env python
# from setuptools import Extension
from setuptools import setup

setup(
    name="pyMARIGOLD",
    version="0.1.1",
    author="Kasper Overgaard Mortensen",
    author_email="km@cs.au.dk",
    packages=["marigold"],
    # scripts=['bin/script1','bin/script2'],
    url="http://github.com/AU-DIS/pyMARIGOLD",
    license="LICENSE",
    description="MARIGOLD for KMeans clustering",
    package_data={"marigold": ["src/marigold.so", "src/marigold.dll"]},
    install_requires=[
        "numpy",
        "pathlib",
    ],
)
