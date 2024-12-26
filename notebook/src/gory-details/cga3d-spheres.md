# Spherelike Objects

## Definition: Sphere in CGA 3D

A **sphere** is an object having a center (represented as a real vector) and a **radius** (a real or pure imaginary number). We encode the sphere in the following way

\\[S(c, r) = c + \frac{1}{2}(c^2 - r^2)\infty + o \\]

In some larger calculations, I may use an abbreviated form of this equation for brevity and substitute in values at the end.

\\[S = A + a\infty + o\\]

This formula is a bit strange, let's break it down:

- The real vector term \\(c\\). The coordinates of the sphere's center are directly represented as \\(c = c_x x + c_y y + c_z z\\)
- The infinity term \\(\frac{1}{2}(c^2 - r^2)\\). This stores information about the distance of the sphere from the origin compared to its radius. This seems strange at first glance. Though in time, we'll find places where this representation helps us compute a sphere inversion.
- The coefficient of \\(\frac{1}{2}\\) feels out of place. This has more to do with the choice of coefficients of the infinity and origin vectors. See [The High-level Basis](./cga3d-basis.md#the-high-level-basis). It's annoying, but better than the alternatives.
- The origin term (homogeneous coordinate) is normalized to be 1. Make sure to do this first before reading off the other terms!.

## Example Spheres

- The **unit sphere** is $$S(0, 1) = -\frac{1}{2}\infty + o = -p = p$$. (See [Inverse Change of Basis](./cga3d-basis.md#inverse-change-of-basis)) TODO: Also link to scalar multiple equivalence. This is interesting, even the low-level basis has a simple geometric meaning!
- The **imaginary unit sphere** is $$S(0, i) = \frac{1}{2}\infty + o = n$$. I don't yet know how to interpret this type of geometric object, but we know that it exists and has an elegant representation!
- Spheres centered on the origin are $$S(0, r) = -\frac{1}{2}r^2\infty + o$$