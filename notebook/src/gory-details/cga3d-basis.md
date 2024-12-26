# 3D CGA Basis

Conformal Geometric Algebra takes a Euclidean geometric algebra and extends it with
two extra dimensions to support projective and inversive geometry.

The construction of CGA is covered in depth in several other sources. For example,
[conformalgeometricalgebra.org](https://www.conformalgeometricalgebra.org/wiki/index.php?title=Main_Page)
gives a detailed introduction. Meanwhile, this document summarizes and
annotates the important definitions.

Frankly, I find the definition of CGA rather inelegant. Adding two extra
dimensions feels arbitrary, and most sources present it without much
explanation. And even when a source takes time to do the full derivation,
it's a long page of algebra just to explain the _basis vectors_. 

However, bearing with the initial confusion is worth it in the long run.
The additional dimensions are just a way to encode more information about the
geometry in a common format. This allows you to represent a wide variety of
geometric transformations (including translations and sphere inversions!) 
without having to treat some as special cases.

## The Low-level Basis

A Euclidean geometric algebra has only positive basis vectors. For example,
2D GA uses the basis \\(x, y\\) and 3D GA uses the basis \\(x, y, z\\). To turn 
this into the corresponding Conformal geometric algebra, we add two
auxillary basis vectors. One with a positive signature (\\(p^2 = 1\\)) and one with 
negative signature (\\(n^2 = -1\\)).

I like to describe these dimensions as either "real" or "auxillary". The
**real** dimensions are dimensions from the original space. Anything we want
to visualize is ultimately projected into this space. I like to refer
to the additional dimensions as **auxillary** dimensions, because we're
adding extra information to make the calculations more uniform. 

<details>
<summary>Alternate terminology from the literature</summary>
There are other
names for this. For example, the <a href="https://clifford.readthedocs.io/en/stable/tutorials/cga/">Python <kbd>clifford</kbd> docs</a>
calls this the <a href="https://en.wikipedia.org/wiki/Minkowski_plane">Minkowski Plane</a>. I'm not familiar with that concept and it adds complexity to the definition.
</details>

Here's a summary of the low-level basis for 2D and 3D CGA. The only difference
between the two is the presence of the z dimension.

| Basis Vector | Notation in literature | Squares to | Description |
| --- | --- |---| --- |
| \\(x\\) | \\(e_1\\) | +1 | Real x dimension |
| \\(y\\) | \\(e_2\\) | +1 | Real y dimension |
| \\(z\\) | \\(e_3\\) | +1 | Real z dimension. (3D CGA only) |
| \\(p\\) | \\(e_+\\) | +1 | Auxillary dimension with positive signature |
| \\(n\\) | \\(e_-\\) | -1 | Auxillary dimension with negative signature |

Adding these two extra dimensions feels rather arbitrary. The way I like
to think about it is that this is the "low-level" description of CGA. This
format can be the easiest to implement in a programming language, but it's
not as human friendly.

If we make a change of basis, we can make something that better matches the
geometry we want to express.

## The High-level Basis

The high-level basis replaces the \\(p, n\\) vectors with a different linear
combination of them:

| Basis Vector | In \\(p, n\\) basis | Squares to | Description |
|---|---|---|---|
| \\(\infty\\) | \\( n + p\\) | 0 | "Infinity" vector. This is associated with the point at infinity as well as stereographic projection. |
| \\(o\\) | \\(\frac{1}{2} (n - p)\\) | 0 | "Origin" vector. This is associated with the origin as well as homogeneous coordinates. |

This is a big cognitive leap! Let's pause here to expand on a few of these
details.

First, why are we adding these two auxillary vectors? Each one enriches the
algebra in a different, but helpful way.

- The \\(o\\) vector acts as a **homogeneous coordinate**. If you have ever worked with computer graphics, this may be familiar! Regardless if you've seen it before, this extra dimension makes it possible to encode translations and other perspective-related details in a consistent format with other transformation types (like rotation).
- The addition of \\(\infty\\) vector makes it possible to do circle/sphere inversions, and other transformations related to **stereographic projection**.

Second, what do these particular dimensions represent?

- The \\(o\\) vector represents the **origin point**
- The \\(\infty \\) vector represents the **point at infinity**
- TODO: Revisit this later once I explore the relationship between points, spheres and their duals.