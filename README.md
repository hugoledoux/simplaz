

[![GitHub license](https://img.shields.io/github/license/hugoledoux/simplaz)](https://github.com/hugoledoux/simplaz/blob/master/LICENSE) [![PyPI version](https://badge.fury.io/py/simplaz.svg)](https://pypi.org/project/simplaz/)

simplaz
=======

A simple Python package to read LAZ files (LAS too).
Basically it's a wrapper around [Rust las](https://docs.rs/las) and it exposes the most useful methods.

Only reading at this moment, writing is for later.


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






