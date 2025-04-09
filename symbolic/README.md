# Symbolic Computations

This directory is a little different. Instead of a Rust package, it's a set
of Python scripts for symbolic computation. This is used to double check
my implementations of math details that would be tedious/error prone to check
by hand. For example, geometric algebra objects have many terms and sign flips, so deriving equations for an implementation would be difficult by hand.

## Dependencies

In your Python environment (virtual environment recommended), you can install
dependencies with

```
pip install -r requirements.txt
```

## Utilities

### Geometric Algebra Products

The script `gaproduct.py` prints out symbolic expressions for each component of
one of the many types of products used in geometric algebra. It also lets you
zoom in on specific types of objects (e.g. compute the wedge product of a bivector and trivector). This is helpful when implementing GA by hand.

Usage:

```sh
python gaproduct.py ALGEBRA PRODUCT_TYPE OBJECT_A OBJECT_B
```

See `python gaproduct.py --help` for the full list of options.

Already I've used this to make an ad-hoc implementation of 2D PGA in [p5-sketchbook](https://github.com/ptrgags/p5-sketchbook)

## Python Preambles

The following scripts set up imports and helper functions for symbolic math
in python. Each one of these is meant to be run as follows:

```sh
python -i preamble_xxx.py
```

Preamble scripts:

- `preamble_pga2d.py` - This script imports `kingdon` for 2D plane-based geometric algebra and defines several helper functions. This is helpful for
debugging math faster