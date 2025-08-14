# Versors

⚠️VERY ROUGH DRAFT

## As transformations

A **versor** is a transformation of space used in Geometric Algebra (GA). Some key properties:

- It is represented as a geometric product of 0 or more reflections, i.e. $V = v_1v_2...v_k$.
- This means versors include any transformation that can be expressed in terms of reflections such as rotations, translations, glide reflections. What transformations are possible depend on which GA is being used.
- Since an reflection is invertible, any product of them will be invertible.
- The magnitude of the versor does not affect the transformation properties, so it is often convenient to work with unit length versors so $V^{-1} = V^\dagger$ (the inverse is the same as the reverse).
- You apply 

IMG: Give some examples of reflections and bireflections?

### Grade of a versor

⚠️Split this section up. I'm talking about 2 concepts here. The number of reflections in a versor, and whether the corresponding grades are odd/even (though the parity of the reflection number are the same.)

- ❓what do you call the grade of a versor?
- a $n$-versor is a versor that's a product of $n$ basis blades. 
    - ❓Is this standard terminology? check the literature
- Sometimes the terminology reflection, bireflection, trireflection, etc. are used. Though this only works for low dimenions
- A versor will always be homogeneous in the following sense
    - All the blades in the multivector represenging a versor will either be all odd or all even.
    - A general reflection is a `vector`. 
    - A general bireflection is a `(scalar + bivector)` (even versor)
    - A general trireflection is a `(vector + trivector)` (odd versor)
    - A general tetrareflection is a `(scalar + bivector + quadvector)` (even versor)
- ⚠️Write about how odd/even versors under multiplication have a form isomorphic to $(\mathbb{Z}_2, +)$

### As a group of transformations

⚠️For this section, we're going to focus on _unit_ versors for ease of
explanation. There are nuances regarding the magnitude of a versor as well
as different signs due to the signature of the algebra. This mainly
accounts for a scaling factor that (usually) cancels out.

Let $V$ be the set of all unit versors $V$, along with the geometric product form a group:

- The group operation represents a _composition_ of transformations. The
    versor $ab$ represents a transformation $b$ followed by the transformation $a$ (⚠️applied right to left like functions!)
- **Closure**: Since we defined a versor as any product of reflections, we
    have guaranteed that versors are closed under composition.
- **Identity**: There is an identity element, $1$, which represents the "do nothing" transformation
- **Invertibility**:
    - Identity $1$ is always a self-inverse by definition.
    - A key property of reflections is that they're self-inverses. $r^2 = 1$ so $r^{-1} = r$. I have a silly mnemonic for this: "upon further reflection, I ended up back where I started".
    - For products of reflections, you undo each reflection in reverse order. i.e. $(ab)^{-1} = b^{-1} a^{-1}
    - So any versor is invertible.
- **associativity**: This is inherited from the associativity of the geometric product. $(ab)c = a(bc)$
- **NO commutativity** 
    - Applying $a$, then $b$ gives a different result than applying $b$, then $a$ in most cases
    - Example: Rotation, then reflection gives a different result than reflection, then rotation. 

IMG: Give _visual_ examples of the above

Since we have the 4 properties of closure, identity, invertibility, associativity, versors form a _group_.

### That pesky minus sign

- The TL;DR - If you are doing a sandwich product where the grades are `odd 🥪 odd`, you need a minus sign out front ($-VxV^{-1}$). In all other cases, 
- In most geometric algebra texts, you'll see some sandwich products written
as $-VxV^{-1}$ but others without. What gives?
- What's happening is that GA reflections are really reflections _through the normal_ not through the reflection plane. Since the orthogonal plane definition
is what we usually think of when we talk about reflection, we need an additional
minus sign out front.
- This is a bit misleading! the minus sign is related to the _filling_ of the sandwich, not the bread! (in fact, if the bread got a minus sign, it would cancel out due to the inverse! $(-V)x(-V)^{-1} = VxV^{-1}$)
- So it's really more like $V(-x)V^{-1}$
- when the filling is a product, you get a negative sign for every vector in the filling. $V(-x)(-y)V^{-1} = VxyV^{-1}$ so the sign wil
- When the bread is a product, you get a negative sign for each reflection in the bread. An even number of these cancel. $B(-(A(-x)A^{-1}))B^{-1} = BAxA^{-1}B^{-1}$
- ⚠️ I have another version of this explanation in my personal notes, is it any clearer? this is messy. 
- ⚠️ This section might be better off in the gorey details version of the article.

IMG: Diagram of reflection through a normal vs through plane

## As geometry

If the basic units of geometric algebra are transformations... where's the
"geometry" in Geometric Algebra?

❓How do you show the connection between the fixed point equation and the OPNS?

REF: what was the name of the 2D CGA book again for citation?

The **outer product null space** is defined as follows.

- Given a transformation $O$, we want to find the set of objects $x$ that are fixed by $O$.
- One way of doing this is with the equation $OxO^{-1} = x$
    - This is equivalent to saying $Ox = xO$, i.e. the values of $x$ that commute with $O$.
    - However, this equation is often not the most convenient to solve
- Another way is to check for objects whose multivector representation are _parallel_ to $O$. 
    - $$O \wedge x = 0$$