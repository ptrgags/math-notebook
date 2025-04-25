use abstraction::group::{Group, GroupAction};

use crate::{isogonal::Isogonal, Complex};

#[derive(Clone)]
pub struct OrbitTile<G, P> {
    pub xform: G,
    pub neighbor_xforms: Vec<G>,
    // Pick one point in the interior of the fundamental domain of the tile.
    // This is used to hash tiles so we don't repeat work.
    // It's important not to pick a point on the boundary, else it might exist
    // in two different tiles at once!
    pub representative: P,
}

impl<G, P> OrbitTile<G, P>
where
    G: Group + GroupAction<P>,
    P: Clone,
{
    pub fn new(xform: G, neighbor_xforms: Vec<G>, representative: P) -> Self {
        Self {
            xform,
            neighbor_xforms,
            representative,
        }
    }

    fn get_neighbor(&self, to_neighbor: G) -> Self {
        // All points in the tile are transformed (including the representative)
        // transform directly
        let xform = to_neighbor.clone() * self.xform.clone();
        let representative = to_neighbor.clone() * self.representative.clone();

        // To find the corresponding arrows in the neighbor tile, we have to
        // conjugate
        let neighbor_xforms: Vec<G> = self
            .neighbor_xforms
            .iter()
            .cloned()
            .map(|x| G::sandwich(to_neighbor.clone(), x))
            .collect();

        Self {
            xform,
            neighbor_xforms,
            representative,
        }
    }

    pub fn get_neighbors(&self) -> Vec<Self> {
        self.neighbor_xforms
            .iter()
            .cloned()
            .map(|xform| self.get_neighbor(xform))
            .collect()
    }
}

pub type IsogonalTile = OrbitTile<Isogonal, Complex>;

#[cfg(test)]
mod test {

    use abstraction::monoid::Monoid;

    use crate::{
        isogonal::Isogonal, isogonal_recipes::reflect_y, point_reflection, translation, Complex,
    };

    use super::*;

    fn make_pmg_xforms() -> [Isogonal; 4] {
        // wallpaper group pmg - see https://en.wikipedia.org/wiki/Wallpaper_group#Group_pmg_(22*)
        // fundamental domain: [0, 1]^2
        // the left and right edges are mirrors
        // the top and bottom edges are 180 degree rotations
        let r180: Isogonal = Isogonal::from(point_reflection());
        let tx: Isogonal = translation(Complex::ONE).unwrap().into();
        let t_top_center = translation(Complex::new(0.5, 1.0)).unwrap().into();
        let t_bottom_center = translation(Complex::new(0.5, 0.0)).unwrap().into();

        let left = reflect_y();
        let right = Isogonal::sandwich(tx, left);
        let up = Isogonal::sandwich(t_top_center, r180);
        let down = Isogonal::sandwich(t_bottom_center, r180);

        [right, up, left, down]
    }

    fn make_pmg_tile() -> OrbitTile<Isogonal, Complex> {
        let tile_center = Complex::new(0.5, 0.5);
        let neighbor_xforms = make_pmg_xforms().to_vec();

        OrbitTile::new(Isogonal::identity(), neighbor_xforms, tile_center)
    }

    #[test]
    fn get_neighbors_with_pmg_group_computes_correct_representatives() {
        let tile = make_pmg_tile();

        let neighbors = tile.get_neighbors();
        let representatives: Vec<Complex> = neighbors.iter().map(|x| x.representative).collect();

        // right, up, left, down
        let expected = vec![
            Complex::new(1.5, 0.5),
            Complex::new(0.5, 1.5),
            Complex::new(-0.5, 0.5),
            Complex::new(0.5, -0.5),
        ];

        for (actual_val, expected_val) in representatives.iter().zip(expected.iter()) {
            assert_eq!(actual_val, expected_val);
        }
    }

    #[test]
    fn get_neighbors_with_pmg_group_computes_correct_transforms() -> Result<(), String> {
        let tile = make_pmg_tile();

        let neighbors = tile.get_neighbors();
        let result: Vec<Isogonal> = neighbors.iter().map(|x| x.xform).collect();

        let expected = make_pmg_xforms().to_vec();

        for (actual_val, expected_val) in result.iter().zip(expected.iter()) {
            assert_eq!(actual_val, expected_val);
        }
        Ok(())
    }

    #[test]
    fn get_neighbors_with_pmg_group_computes_correct_neighbor_transforms() {
        let tile = make_pmg_tile();

        let neighbors = tile.get_neighbors();
        let result: Vec<Vec<Isogonal>> = neighbors
            .iter()
            .map(|x| x.neighbor_xforms.clone())
            .collect();

        let [right, up, left, down] = make_pmg_xforms();
        let expected = vec![
            // Neighbor to right
            vec![
                right,
                Isogonal::sandwich(right, up),
                Isogonal::sandwich(right, left),
                Isogonal::sandwich(right, down),
            ],
            // neighbor above
            vec![
                Isogonal::sandwich(up, right),
                up,
                Isogonal::sandwich(up, left),
                Isogonal::sandwich(up, down),
            ],
            vec![
                Isogonal::sandwich(left, right),
                Isogonal::sandwich(left, up),
                left,
                Isogonal::sandwich(left, down),
            ],
            vec![
                Isogonal::sandwich(down, right),
                Isogonal::sandwich(down, up),
                Isogonal::sandwich(down, left),
                down,
            ],
        ];

        assert_eq!(result.len(), expected.len());
        for (actual_neighbors, expected_neighbors) in result.iter().zip(expected.iter()) {
            assert_eq!(actual_neighbors.len(), expected_neighbors.len());
            for (actual_val, expected_val) in actual_neighbors.iter().zip(expected_neighbors.iter())
            {
                assert_eq!(actual_val, expected_val);
            }
        }
    }
}
