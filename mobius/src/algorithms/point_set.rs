use std::collections::HashMap;

use crate::{quantized_hash::QuantizedHash, Complex};

pub trait Set<V>: Default {
    #[cfg(test)]
    fn len(&self) -> usize;
    fn contains(&self, value: &V) -> bool;
    fn insert(&mut self, value: V);
}

pub struct PointSet {
    grid: HashMap<(isize, isize), Vec<Complex>>,
    quantize_bits: i32,
}

impl PointSet {
    pub fn new(quantize_bits: i32) -> Self {
        Self {
            grid: HashMap::new(),
            quantize_bits,
        }
    }

    fn cell_contains(&self, cell_id: (isize, isize), value: Complex) -> bool {
        let maybe_values = self.grid.get(&cell_id);

        if let Some(values) = maybe_values {
            values.iter().any(|x| *x == value)
        } else {
            false
        }
    }
}

impl Default for PointSet {
    fn default() -> Self {
        Self::new(8)
    }
}

impl Set<Complex> for PointSet {
    #[cfg(test)]
    fn len(&self) -> usize {
        self.grid.values().map(|x| x.len()).sum()
    }

    fn contains(&self, value: &Complex) -> bool {
        // Get the grid cell
        let cell = value.quantize(self.quantize_bits);

        if self.cell_contains(cell, *value) {
            return true;
        }

        // If the point is on the edge of a bin, float error might bump it
        // into a neighbor cell. So look at the 8 neighbors and check for equivalent
        // points
        let (x0, y0) = cell;
        for x in (x0 - 1)..=(x0 + 1) {
            for y in (y0 - 1)..=(y0 + 1) {
                let neighbor = (x, y);
                if neighbor == cell {
                    continue;
                }

                if self.cell_contains(neighbor, *value) {
                    return true;
                }
            }
        }

        false
    }

    fn insert(&mut self, value: Complex) {
        // The point is already in the set, we don't have to do anything
        if self.contains(&value) {
            return;
        }

        let cell = value.quantize(self.quantize_bits);
        self.grid.entry(cell).or_default().push(value);
    }
}

#[cfg(test)]
mod test {
    use crate::nearly::EPSILON;

    use super::*;

    #[test]
    pub fn inserts_point() {
        let point = Complex::new(1.0, 2.0);
        let mut set = PointSet::default();

        set.insert(point);

        assert!(set.contains(&point));
        assert_eq!(set.len(), 1);
    }

    #[test]
    pub fn insert_is_idempotent() {
        let point = Complex::new(1.0, 2.0);
        let mut set = PointSet::default();

        set.insert(point);
        set.insert(point);

        assert_eq!(set.len(), 1);
    }

    #[test]
    pub fn equivalent_points_are_deduplicated() {
        let point = Complex::new(1.0, 2.0);
        let slightly_off = point + Complex::from(EPSILON / 2.0);
        let mut set = PointSet::new(4);

        assert_eq!(point, slightly_off);

        set.insert(point);
        set.insert(slightly_off);

        assert!(set.contains(&point));
        assert!(set.contains(&slightly_off));
        assert_eq!(set.len(), 1);
    }
}
