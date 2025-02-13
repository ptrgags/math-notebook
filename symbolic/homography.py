# We want to transform the quad ABCD into the quad EFGH
import sympy

# 4 input points
Ax, Ay = sympy.symbols('Ax Ay')
Bx, By = sympy.symbols('Bx By')
Cx, Cy = sympy.symbols('Cx Cy')
Dx, Dy = sympy.symbols('Dx Dy')

# 4 output points
Ex, Ey = sympy.symbols('Ex Ey')
Fx, Fy = sympy.symbols('Fx Fy')
Gx, Gy = sympy.symbols('Gx Gy')
Hx, Hy = sympy.symbols('Hx Hy')

# each output is only defined up to a scale
p, q, r, s = sympy.symbols('p q r s')

# Homography coefficients
a, b, c, d, e, f, g, h, i = sympy.symbols("a b c d e f g h i")

H = sympy.Matrix([
    [a, b, c],
    [d, e, f],
    [g, h, i],
])

uv_input = sympy.Matrix([
    [0, 1, 1, 0],
    [0, 0, 1, 1],
    [1, 1, 1, 1]
])

# With this input the solving takes very long. so let's not for now.
# If we can compute the transformation and its inverse, we can compute the
# full homography from two UV homographies.
input = sympy.Matrix([
    [Ax, Bx, Cx, Dx],
    [Ay, By, Cy, Dy],
    [1, 1, 1, 1]
])

output = sympy.Matrix([
    [p * Ex, q * Fx, r * Gx, s * Hx],
    [p * Ey, q * Fy, r * Gy, s * Hy],
    [p, q, r, s],
])
sympy.pprint(H * uv_input - output)

coordinate_constraints = H * uv_input - output
det_constraint = H.det() - 1

results = sympy.solve(coordinate_constraints, [
                      a, b, c, d, e, f, g, h, i, p, q, r])


sympy.pprint(sympy.simplify(results[a]/s))

# The determinant of this matrix is the denominator for a (and maybe other coefficients?)
efg = sympy.Matrix([
    [Ex, Fx, Gx],
    [Ey, Fy, Gy],
    [1, 1, 1]
])
denominator = efg.det()

'''
new_s = sympy.solve(det_constraint.subs(results), s)
print(len(new_s))
sympy.pprint(sympy.simplify(new_s[0]))
'''

for var, expr in results.items():
    factor_out_s = sympy.collect(expr, s)
    numerator = sympy.simplify(factor_out_s * denominator / s)
    sympy.pprint(var)
    sympy.pprint(numerator)

# solve() is incredibly slow when you put all the equations together.
#
# equations = list(coordinate_constraints) + [det_constraint]
# print("solving...")
# results = sympy.solve(equations, [a, b, c, d, e, f, g, h, i, p, q, r, s])
# print("printing...")
# sympy.pprint(results)
