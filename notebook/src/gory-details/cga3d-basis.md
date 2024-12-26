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
combination of them to create null vectors (vectors that square to 0)

| Basis Vector | In \\(p, n\\) basis | Squares to | Description |
|---|---|---|---|
| \\(o\\) | \\(\frac{1}{2} (n - p)\\) | 0 | "Origin" vector. This is associated with the origin as well as homogeneous coordinates. |
| \\(\infty\\) | \\( n + p\\) | 0 | "Infinity" vector. This is associated with the point at infinity as well as stereographic projection. |

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

Finally, what's the deal with the coefficient of \\(1/2\\) in one of the two
definitions? It feels strange and asymmetric. In [conformalgeometricalgebra.org](https://www.conformalgeometricalgebra.org/wiki/index.php?title=Main_Page) 
again, it explains that the coefficients are arbitrary, and this is the nicest choice possible. Specifically:

- It's convenient to have \\(\infty \wedge o = p \wedge n \\). To do this this means the product of coefficients must be 1/2. TODO: Proof?
- A more symmetric choice would be to use \\( \left(\frac{1}{\sqrt{2}}, \frac{1}{\sqrt{2}}\right) \\). This is not ideal especially when using floating point math. \\((1/2, 1)\\) is more computer-friendly.
- As for which term gets the \\(1/2\\)... pick your poison I guess?

## Inverse Change of Basis

I've yet to see a text that mentions the formulas for the other direction, going
from high-level to low-level. Sometimes this is helpful to simplify equations,
so here they are for reference:

$$
\begin{align}
    p &= \frac{1}{2}\infty - o\\\\
    n &= \frac{1}{2}\infty + o \\\\
\end{align}
$$


## Partial Multiplication Tables

For reference, here are the multiplication tables for the high-level basis, as
there will be a lot of calculations using these. 

### Dot Product

| \\(a \cdot b \\)| \\(x\\) | \\(y\\) | \\(z\\) | \\(\infty \\) | \\( o \\) |
|---|---|---|---|---|---|
| \\(x\\)       | 1 | 0 | 0 | 0  | 0 |
| \\(y\\)       | 0 | 1 | 0 | 0  | 0 | 
| \\(z\\)       | 0 | 0 | 1 | 0  | 0 |
| \\(\infty \\) | 0 | 0 | 0 | 0  | -1 | 
| \\( o \\)     | 0 | 0 | 0 | -1 | 0 |

Observations:

- The dot product of two real basis vectors is either 1 if the vectors are parallel, or 0 otherwise.
- The dot product of a real basis vector and an auxillary basis vector is always 0.
- The dot product of the auxillary vectors is unusual. The product is -1 only if the vectors are _perpendicular_! This is backwards from the typical case!
- The dot product is commutative (even for auxillary vectors). Notice that entries are the same across the main diagonal.

### Wedge Product

| \\(a \wedge b \\)| \\(x\\) | \\(y\\) | \\(z\\) | \\(\infty \\) | \\( o \\) |
|---|---|---|---|---|---|
| \\(x\\)       | 0              | \\(xy\\)        | \\(xz\\)        | \\(x\infty\\)  | \\(xo\\)       |
| \\(y\\)       | \\(-xy\\)      | 0               | \\(yz\\)        | \\(y\infty\\)  | \\(yo\\)       | 
| \\(z\\)       | \\(-xz\\)      | \\(-yz\\)       | 0               | \\(z\infty\\)  | \\(zo\\)       |
| \\(\infty \\) | \\(-x\infty\\) | \\(-\infty x\\) | \\(-\infty x\\) | 0              | \\(\infty o = E_0 = pn\\) | 
| \\( o \\)     | \\(-xo\\)      | \\(-yo\\)       | \\(-zo\\)       | \\(- \infty o = -E_0 = -pn\\)      | 0              |

Observations:

- If the vectors are parallel, the wedge product is 0.
- For every other product, combine the two vectors into a bivector. Swap vectors around (introducing a minus sign for every swap) until the vectors are in order. I like to sort the vectors following this order: \\(x, y, z, \infty,  o\\).
- The wedge product is anticommutative. Notice that the entries are negated across the main diagonal of the table. 

### Geometric Product

The geometric product of vectors is the sum of the dot and wedge products. \\(ab = a \cdot b + a \wedge b\\).
Note that this is true for vectors, but it changes a bit for other types of blades.

| \\(ab \\)| \\(x\\) | \\(y\\) | \\(z\\) | \\(\infty \\) | \\( o \\) |
|---|---|---|---|---|---|
| \\(x\\)       | 1              | \\(xy\\)        | \\(xz\\)        | \\(x\infty\\)  | \\(xo\\)       |
| \\(y\\)       | \\(-xy\\)      | 1               | \\(yz\\)        | \\(y\infty\\)  | \\(yo\\)       | 
| \\(z\\)       | \\(-xz\\)      | \\(-yz\\)       | 1               | \\(z\infty\\)  | \\(zo\\)       |
| \\(\infty \\) | \\(-x\infty\\) | \\(-\infty x\\) | \\(-\infty x\\) | 0              | \\(-1 + E_0\\) | 
| \\( o \\)     | \\(-xo\\)      | \\(-yo\\)       | \\(-zo\\)       | \\(-1 - E_0\\) | 0              |

Observations:

- It's important to note that \\((\infty) (o) \ne \infty \wedge o = \infty o\\). There will always be both dot and wedge terms! This is why I use extra parentheses for the geometric product when it may be ambiguous.
- For any pair of _orthogonal_ basis vectors, the geometric product is equal to the wedge product and thus can be used interchangeably.
- For any pair of _parallel_ basis vectors, the geometric product are the same as the dot product and thus can be used interchangeably.
- This means that in all cases except \\((\infty) (o)\\):
    - _parallel_ vectors commute. \\(ab = ba\\)
    - _orthogonal_ vectors anticommute. \\(ab = -ba\\)
    - For vectors that are not parallel nor orthogonal, there will be both dot and wedge terms.


### Sandwich Product

One product we'll see a lot is the sandwich product, as this is how we apply transformations. For vectors, \\(ab\tilde{a} = aba\\) since vectors are their own reverse.

| \\(aba \\)| \\(x\\) | \\(y\\) | \\(z\\) | \\(\infty \\) | \\( o \\) |
|---|---|---|---|---|---|
| \\(x\\)       | \\(x\\)  | \\(-y\\) | \\(-z\\) | \\(-\infty\\)  | \\(-o\\) |
| \\(y\\)       | \\(-x\\) | \\(y\\)  | \\(-z\\) | \\(-\infty\\)  | \\(-o\\) | 
| \\(z\\)       | \\(-x\\) | \\(-y\\) | \\(z\\)  | \\(-\infty\\)  | \\(-o\\) |
| \\(\infty \\) | 0 | 0 | 0 | 0                          | \\(-2\infty = \infty\\) (normalized) | 
| \\( o \\)     | 0 | 0 | 0 | \\(-2o = o\\) (normalized) | 0              |

Observations:

- Real basis vectors act as reflections through that vector. It fixes the parallel direction, and negates everything else. 
- \\(\infty\\) does something a little different. It collapses the entire space down to the point at infinity. TODO: is this a correct interpretation?
- \\(o\\) is similar to \\(\infty\\) except it collapses down to the origin instead of infinity. TODO: Is this also true for spheres of radius 0 in general?