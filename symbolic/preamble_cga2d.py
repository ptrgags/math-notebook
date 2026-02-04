import math
import kingdon
alg = kingdon.Algebra(3, 1, 0)


def even(scalar, xy, xp, xm, yp, ym, pm, xypm):
    return alg.multivector(e=scalar, e12=xy, e13=xp, e14=xm, e23=yp, e24=ym, e34=pm, e1234=xypm)


def odd(x, y, p, m, xyp, xym, xpm, ypm):
    return alg.multivector(e1=x, e2=y, e3=p, e4=m, e123=xyp, e124=xym, e134=xpm, e234=ypm)


def lerp(a, b, t):
    return (1.0 - t) * a + t * b
