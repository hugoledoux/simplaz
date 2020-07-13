

[![GitHub license](https://img.shields.io/github/license/hugoledoux/simplaz)](https://github.com/hugoledoux/simplaz/blob/master/LICENSE) [![PyPI version](https://badge.fury.io/py/simplaz.svg)](https://pypi.org/project/simplaz/)

simplaz
=======

A simple Python package to read LAZ files (LAS too).
Basically it's a wrapper around [Rust las](https://docs.rs/las) and it exposes the most useful methods.

Only reading at this moment; writing is for later.


Installation
============

pip
---

To install the latest release: `pip install simplaz`


Development
-----------

  1. install [Rust](https://www.rust-lang.org/) (v1.39+)
  2. install [maturin](https://github.com/PyO3/maturin) 
  3. `maturin development`
  4. move to another folder, and `import simplaz` shouldn't return any error


Example
=======

```python
import simplaz

ds = simplaz.read_file("/home/elvis/myfile.laz")

header = ds.header
print("LAS v{}".format(header.version))
print("Point count: {}".format(header.number_of_points))

#-- iterate over all the points
count_ground = 0
for point in ds:
    if point.classification == 2:
        count_ground += 1
print("Total ground points: {}".format(count_ground))
```


What is supported and what not?
===============================

Most of [LAS v1.4](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf) is supported, except:


LAS classes
===========

| Classification | description                   | 
| -------------- | ----------------------------- |
|  0             | Created, never classified     |
|  1             | Unclassfied                   |
|  2             | Ground                        |
|  3             | Low Vegetation                |
|  4             | Medium Vegetation             |
|  5             | High Vegetation               |
|  6             | Building                      |
|  7             | Low Point (noise)             |
|  8             | Model Key-point (mass point)  |
|  9             | Water                         |
| 10             | Reserved for ASPRS definition |
| 11             | Reserved for ASPRS definition |
| 12             | Overlap Points                |
| 13-31          | Reserved for ASPRS definition |
