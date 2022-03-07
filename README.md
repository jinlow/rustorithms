# rustorithms
## Exploring Algorithms With Rust
Implementing different algorithms in rust. The algorithms are common ones, where I have mainly been using the books Grokking Algorithms by Bhargava, and then Intro to Algorithms by Cormen, Rivest, Leiserson, and Stein for reference.

The following algorithms have been implemented.
  - Binary Search
  - Linked List
  - Selection Sort
  - Insertion Sort
  - Merge Sort
  - Quick Sort
  - Hashtable

### Trying yourself...
Each algorithm is implemented as a separate rust crate.
Examples of all of the algorithms are provided in the `examples/` directory. To run them, clone this repo, and the examples can be run with cargo:
```shell
> cd rustorithms
> cargo run --example quicksort
```

### Python Bindings
_Why not!_  
Additionally Python binding are being implemented for some of the algorithms in the `pyorithms` directory. There is a `setup.py` file in there that can be used to test out the Python library.
For this the package `setuptools_rust` is required, as well as a working rust compiler. From inside the `pyorithms` directory you can run the following to build the Python bindings, and create an
importable python library you can test out:
```shell
> python setup.py develop
```
For more information take a look at the [pyorithms README](pyorithms/)
