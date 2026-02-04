## Circular Arcs

Sometimes, I want to draw just a portion of a circle. It turns out that there
are several ways to express this. Let's compare them.

## Circle and Signed Angles

I think of a circular arc as a portion of a circle defined by two angles
that determine where it starts and ends.

IMG: Diagram of circular arc

In other words, the tuple:

```
CircularArc(circle: Circle, angles: ArcAngles)
```

where: 

- `circle` is the `(center, radius)` of the circle on
which the arc lives
- `angles` is a pair `(start_angle, end_angle)`, representing an oriented arc from `start_angle` to `end_angle`. I use the following restrictions as a "canonical form":
    - `start_angle` must be in $[0, 2\pi)$
    - `end_angle` must be within a full circle of `start_angle`. I.e. $|end - start| \le 2\pi$. Note that this includes the upper bound to allow expressing a full circle. 

### Properties

- Equality - Given the restrictions on the angles, you can simply test if the circle and angles of one arc are equal (up to floating point epsilon).
- `arc.orientation` - Which direction does the arc curve? Given the restrictions above, this is simple to compute
    - Compute `sign(end_angle - start_angle)`
    - +1 means positive angle
    - 0 means start_angle = end_angle, i.e. the angle is 0
    - -1 means negative angle
- I describe the arc in terms of "positive/negative" angles. Whether these
are clockwise/counterclockwise depends on the coordinate system used by the
graphics library.
    - In y-up coordinate systems, positive is usually counterclockwise
    - In y-down coordinate systems, positive is usually _clockwise_

### Transformations

- `arc.flip_y() = (-start_angle, -end_angle)` - to switch between y-up and 
y-down coordinate systems, simply negate the angles. 
- `arc.reverse() = reduce(end_angle, start_angle)` - To swap the endpoints, simply reverse the angles and reduce so the new start angle is in $[0, 2\pi)$

## p5.js

TODO

## PostScript

TODO


## SVG 

TODO