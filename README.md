# rustorithms
## Exploring Grokking Algorithms With Rust
Working through examples from the book Grokking Algorithms by Aditya Bhargava. This repo is largely to explore Rust a little more.

The following algorithms have been implemented using Rust as examples of topics inspired by the book.
  - Chapter 1
    + Binary Search
  - Chapter 2
    + Linked List
    + Selection Sort
  - Chapter 3
    + Recursion
  - Chapter 4
    + Quicksort
  - Chapter 5
    + Hashtable

### Trying yourself...
Each algorithm is implemented as a separate rust crate. The code all was built using rust 1.46.0.  
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
For more information take a look at the [pyorithms README](pyorithms/README.md)
