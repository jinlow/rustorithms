import sys

from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pyorithms",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Rust",
    ],
    rust_extensions=[RustExtension("pyorithms.pyorithmslib")],
    packages=["pyorithms"],
    include_package_data=True,
    zip_safe=False,
)