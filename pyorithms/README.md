Pyorithms
===========
Python Bindings for Rustorithms
------------------------------

This package has python bindings to the rustorithm algorithms.  

_Why would you need a library of python bindings to algorithms that are already implemented everywhere?_  
You probably don't... This project is, more or less, just for kicks. It also serves as a POC for implementing things that can maybe be done faster in rust, than if they were built in pure python. Also it's fun 😃
 
------------
Installation
------------
The easiest way to install this package is directly from this repo subfolder. The prerequisites are you need to have a working rust compiler installed ([Rust install instructions](https://www.rust-lang.org/tools/install)), as well as the python package `setuptools-rust`.  
```shell
> pip install setuptools-rust
```
After that you can clone the rustorithms repo then run either of the following two commands.  
_Create a debug build for testing_
```shell
> python setup.py develop
```
_Install the package_
```shell
> python setup.py install
```

-----
Usage
-----
Right now the only algorithms implemented in the package are quicksort, and a hash table implementation.
```python
import pyorithms as ps

# The quicksort algorithm can take a list of strings,
# floats and integers
ps.qsort([3, 2, 1])
# Returns: [1, 2, 3]

# There is also a HashTable class that is similar to
# the python Dict object. It is more limited though,
# because it can only take integers and strings as keys,
# and then accept strings, floats or integers as values.
ht = ps.HashTable()

# Add some items
ht.add("k1", 1)
ht.add(10, 100)

# Get a value
ht.get("k1", 1)
# Returns: 1

ht
# Returns:
# Hashtable Data:
#   'k1', 1
#   10, 100

# Delete a value
ht.delete("k1")

ht
# Returns:
# Hashtable Data:
#   'k1', 1
```

A simple, likely meaningless, benchmark on my machine... Because why not.
```python
import pyorithms as ps
from statistics import stdev, mean
import time
from typing import List
import random

random.seed(123)
rand_ints = [random.choice(range(10_000)) for i in range(300_000)]

RUNS = 300

qsort_time: List[float] = []
for i in range(RUNS):
    start = time.time()
    ps.qsort(rand_ints)
    qsort_time.append(time.time() - start)

sorted_time: List[float] = []
for i in range(RUNS):
    start = time.time()
    sorted(rand_ints)
    sorted_time.append(time.time() - start)

measures = ["Min", "Max", "Mean", "Stdev"]
print("      Function: " + "".join(f"{m:<9}" for m in measures))
print(
    f"ps.quicksort(): {min(qsort_time):.5f}  "
    + f"{max(qsort_time):.5f}  "
    + f"{mean(qsort_time):.5f}  "
    + f"{stdev(qsort_time):.5f}  "
)

print(
    f"sorted()      : {min(sorted_time):.5f}  "
    + f"{max(sorted_time):.5f}  "
    + f"{mean(sorted_time):.5f}  "
    + f"{stdev(sorted_time):.5f}  "
)

# Returns:
#       Function: Min      Max      Mean     Stdev
# ps.quicksort(): 0.02912  0.03965  0.02991  0.00081
# sorted()      : 0.04107  0.04750  0.04186  0.00067
```

So our quicksort algorithm is a little faster on this contrived example.
