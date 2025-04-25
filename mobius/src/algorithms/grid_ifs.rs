use abstraction::{semigroup::Semigroup, Group};

use crate::transformable::Transformable;

/// Descriptor (g, start_power, end_power) for an "axis", i.e. a single group
/// element raised to a sequence of powers from [start_power, end_power)
pub type AxisDescriptor<G> = (G, isize, isize);

/// Internal representation for one of the grid axes
/// relative to the minimum power, which means it can iterate
/// over unsigned integers rather than signed ones which complicate
/// things.
struct Axis<G> {
    xform: G,
    start: G,
    iters: usize,
}

//let (xform, start_power, end_power) =

impl<G: Group> From<AxisDescriptor<G>> for Axis<G> {
    fn from(value: AxisDescriptor<G>) -> Self {
        let (xform, start_power, end_power) = value;
        let start = Group::pow(&xform, start_power);
        let iters = (end_power - start_power) as usize;

        Self {
            xform,
            start,
            iters,
        }
    }
}

pub struct GridIFS<G: Group> {
    axes: Vec<Axis<G>>,
}

impl<G: Group> GridIFS<G> {
    pub fn new(axis_descriptors: Vec<AxisDescriptor<G>>) -> Self {
        let axes: Vec<Axis<G>> = axis_descriptors.into_iter().map(|x| x.into()).collect();
        Self { axes }
    }

    pub fn dimensions(&self) -> usize {
        self.axes.len()
    }

    pub fn iter(&self) -> GridIFSIterator<G> {
        GridIFSIterator::new(self)
    }

    pub fn conjugate(&self, transform: G) -> Self {
        let axes: Vec<Axis<G>> = self
            .axes
            .iter()
            .map(
                |Axis {
                     xform,
                     start,
                     iters,
                 }| Axis {
                    xform: G::sandwich(transform.clone(), xform.clone()),
                    start: G::sandwich(transform.clone(), start.clone()),
                    iters: *iters,
                },
            )
            .collect();
        Self { axes }
    }

    pub fn apply<T: Transformable<G>>(&self, primitive: &T) -> Vec<T> {
        self.iter()
            .map(|(_, xform)| primitive.transform(xform))
            .collect()
    }

    /// When T values can be combined, this method is convenient for flattening
    /// the results of apply() into a single T.
    pub fn flat_apply<T>(&self, primitive: &T) -> T
    where
        T: Transformable<G> + Semigroup,
    {
        let applied = self.apply(primitive);
        Semigroup::sconcat(&applied)
    }
}

pub struct GridIFSIterator<'a, G: Group> {
    ifs: &'a GridIFS<G>,
    current_indices: Vec<usize>,
    current_values: Vec<G>,
    stop: bool,
}

impl<'a, G: Group> GridIFSIterator<'a, G> {
    fn new(ifs: &'a GridIFS<G>) -> Self {
        let current_values: Vec<G> = ifs.axes.iter().map(|x| x.start.clone()).collect();
        Self {
            ifs,
            current_indices: vec![0; ifs.dimensions()],
            current_values,
            stop: false,
        }
    }
}

impl<'a, G: Group> Iterator for GridIFSIterator<'a, G> {
    type Item = (Vec<usize>, G);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let indices = self.current_indices.clone();
        let element = self
            .current_values
            .iter()
            .cloned()
            .reduce(|prod, element| prod * element)
            .unwrap();

        // increment the index counter from right to left
        // This is much like a ripple carry adder, but we're incrementing
        // the powers for each axis
        for i in (0..self.ifs.dimensions()).rev() {
            let index = self.current_indices[i];
            let n = self.ifs.axes[i].iters;

            if index == n - 1 && i == 0 {
                // We're at the last iteration, we can stop.
                self.stop = true;
            } else if index == n - 1 {
                // roll over to the next place
                self.current_indices[i] = 0;
                self.current_values[i] = self.ifs.axes[i].start.clone();
            } else {
                // implement the power for this element only
                self.current_indices[i] += 1;
                self.current_values[i] =
                    self.current_values[i].clone() * self.ifs.axes[i].xform.clone();
                break;
            }
        }

        Some((indices, element))
    }
}
