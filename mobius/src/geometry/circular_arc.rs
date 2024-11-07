use std::{f64::consts::TAU, fmt::Display};

use crate::Complex;

use super::{circle::Circle, ArcAngles, ArcAnglesParseError, ArcDirection, DirectedEdge, Geometry};

fn compute_ccw_angles(a: f64, b: f64, c: f64) -> Result<ArcAngles, ArcAnglesParseError> {
    // Compute angles in the CCW direction
    let delta_b = (b - a).rem_euclid(TAU);
    let delta_c = (c - b).rem_euclid(TAU);

    let adjusted_b = a + delta_b;
    let adjusted_c = adjusted_b + delta_c;
    ArcAngles::new(a, adjusted_c)
}

fn compute_cw_angles(a: f64, b: f64, c: f64) -> Result<ArcAngles, ArcAnglesParseError> {
    // Compute angles in the CW direction
    let delta_b = (a - b).rem_euclid(TAU);
    let delta_c = (b - c).rem_euclid(TAU);

    let adjusted_b = a - delta_b;
    let adjusted_c = adjusted_b - delta_c;
    ArcAngles::new(a, adjusted_c)
}

// Directed circular arc through 3 points on a circular arc
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub angles: ArcAngles,
}

impl CircularArc {
    pub fn new(circle: Circle, angles: ArcAngles) -> Self {
        Self { circle, angles }
    }

    /// Given a circle and 3 angles (computed )
    pub fn from_circle_and_points(
        circle: Circle,
        a: Complex,
        b: Complex,
        c: Complex,
    ) -> Result<Self, ArcAnglesParseError> {
        // Determine if the 3 points circulate counterclockwise or
        // clockwise by forming a triangle ABC and computing
        // (the magnitude of) the wedge product.
        let ab = b - a;
        let ac = c - a;
        let ccw = Complex::wedge(ab, ac) > 0.0;

        // Get the raw angles
        let theta_a = circle.get_angle(a).unwrap();
        let theta_b = circle.get_angle(b).unwrap();
        let theta_c = circle.get_angle(c).unwrap();

        let angles = if ccw {
            compute_ccw_angles(theta_a, theta_b, theta_c)
        } else {
            compute_cw_angles(theta_a, theta_b, theta_c)
        }?;

        Ok(Self { circle, angles })
    }

    pub fn direction(&self) -> ArcDirection {
        self.angles.direction()
    }

    pub fn midpoint(&self) -> Complex {
        let ArcAngles(a, c) = self.angles;
        let b = 0.5 * (a + c);

        self.circle.get_point(b)
    }

    pub fn complement(&self) -> Self {
        Self {
            circle: self.circle,
            angles: self.angles.complement(),
        }
    }
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        let ArcAngles(a, _) = self.angles;
        self.circle.get_point(a)
    }

    fn end(&self) -> Complex {
        let ArcAngles(_, b) = self.angles;
        self.circle.get_point(b)
    }
}

impl Display for CircularArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CircularArc {
            circle: Circle { center, radius },
            angles,
        } = self;
        write!(f, "Arc(c={}, r={:.3}, {})", center, radius, angles)
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn missing_tests() {
        todo!("from_circle_and_points, directed_edge");
    }
}
