# 3D CGA

## It's All Spheres

In 3D CGA, the basic primitive is a **sphere**. A sphere has a **center**
and a **radius**. 

IMG: Diagram of a sphere

A sphere represents a type of reflection known as a [sphere inversion](https://en.wikipedia.org/wiki/Inversive_geometry#In_three_dimensions). This swaps the inside of the
sphere with its outside. Note that the sphere itself remains fixed.

IMG: Diagram of inversion

As with any reflection, this is an **involution**, i.e. repeating this
transformation twice is the same as doing nothing.

IMG: Diagram of reflecting twice

## Special Cases of a Sphere

- The **unit sphere** is a sphere with center at the origin and radius 1
    - As a reflection, this has a nice property that a point maps to its reciprocal
    - Also, this swaps the origin and the point at infinity.
- A sphere can have an imaginary radius. I'm not sure how exactly to interpret this yet.
- Imagine taking a sphere and reducing the radius all the way to 0.
    - On one hand, this feels like a "point"
    - However, there isn't really an "inside" of the point anymore, only the point itself. My conjecture is that this is something along the lines of "collapse the entire space down to the point". It also reminds me of the undefined product of the real numbers \\(0, \infty \\)
- There are two special degenerate spheres - the origin, and the point at infinity.
- Imagine taking the sphere and making the radius bigger without bound and zooming in on the surface. It will flatten out into a **plane**.