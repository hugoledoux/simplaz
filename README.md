
simplaz
=======

A simple Python package to read LAZ files (LAS too).
Basically it's a wrapper around [Rust las](https://docs.rs/las) and it exposes the most useful methods.

Only reading at this moment, writing is for later.


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






