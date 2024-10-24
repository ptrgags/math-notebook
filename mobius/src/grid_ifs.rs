use std::ops::Range;

use abstraction::Group;

/// Iterate a grid of transformations 
pub struct GridIFS<G: Group> {
    a: G,
    b: G,
    a_range: Range<isize>,
    b_range: Range<isize>,
}

impl<G: Group> GridIFS<G> {
    pub fn new(a: G, b: G, a_range: Range<isize>, b_range: Range<isize>) -> Self {
        Self {
            a, b, a_range, b_range
        }
    }

    /*
    pub fn iter(&self) -> impl Iterator<Item=G> {
        // Handling negative ranges will be a pain
        let depth = self.a_range.end as usize;
        let a_forward = self.a.power_iter().take(depth);
        let a_backward = self.a.inv_power_iter().skip(1).take(depth);

        a_forward.chain(a_backward).flat_map(|a| {
            let b_depth = self.b_range.end as usize;
            let b_forward = self.b.power_iter().take(b_depth);
            let b_backward = self.b.inv_power_iter().skip(1).take(depth);
            b_forward.chain(b_backward).map(|b| a * b)
        })
    }
    */
}