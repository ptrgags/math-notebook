import math
import kingdon
alg = kingdon.Algebra(3, 0, 1)


def even(scalar, xy, xz, xo, yz, yo, zo, xyzo) -> kingdon.MultiVector:
    return alg.multivector(e=scalar, e12=xy, e13=xz, e01=-xo, e23=yz, e02=-yo, e03=-zo, e0123=-xyzo)


def odd(x: float, y: float, z: float, o: float, xyz: float, xyo: float, xzo: float, yzo: float) -> kingdon.MultiVector:
    return alg.multivector(e1=x, e2=y, e3=z, e0=o, e123=xyz, e012=xyo, e013=xzo, e023=yzo)


def point(x: float, y: float, z: float) -> kingdon.MultiVector:
    return alg.trivector(e123=1, e023=x, e023=y, e012=z)

# TODO: Still need to add more functions, see preamble_pga2d.py
