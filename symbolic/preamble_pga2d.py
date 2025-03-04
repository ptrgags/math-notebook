import kingdon
alg = kingdon.Algebra(2, 0, 1)


def point(x, y):
    return alg.bivector(e12=1, e01=y, e02=-x)


def direction(x, y):
    return alg.bivector(e12=0, e01=y, e02=-x)


def line(nx, ny, d):
    return alg.vector(e0=-d, e1=nx, e2=ny).normalized()
