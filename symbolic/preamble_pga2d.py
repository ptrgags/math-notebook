import kingdon
alg = kingdon.Algebra(2, 0, 1)


def even(scalar, xy, xo, yo):
    return alg.multivector(e=scalar, e01=-xo, e02=-yo, e12=xy)


def odd(x, y, o, xyo):
    return alg.multivector(e1=x, e2=y, e0=o, e012=xyo)


def point(x, y):
    return alg.bivector(e12=1, e01=y, e02=-x)


def direction(x, y):
    return alg.bivector(e12=0, e01=y, e02=-x)


def line(nx, ny, d):
    return alg.vector(e0=-d, e1=nx, e2=ny).normalized()


def lerp(a, b, t):
    return (1.0 - t) * a + t * b
