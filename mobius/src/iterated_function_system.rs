use crate::{cline_tile::ClineArcTile, Mobius};

/// Iterated Function System. This is still in a prototype stage
pub struct IFS {
    xforms: Vec<Mobius>,
}

impl IFS {
    pub fn new(xforms: Vec<Mobius>) -> Self {
        Self { xforms }
    }

    pub fn get_xform(&self, index: usize) -> Mobius {
        self.xforms[index]
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Mobius> {
        self.xforms.iter()
    }

    pub fn dfs(&self, max_depth: usize) -> IFSDepthFirstIterator {
        IFSDepthFirstIterator::new(self, max_depth)
    }

    pub fn sandwich(bread: Mobius, ifs: &Self) -> Self {
        Self {
            xforms: ifs
                .xforms
                .iter()
                .map(|x| Mobius::sandwich(bread, *x))
                .collect(),
        }
    }
}

pub struct IFSDepthFirstIterator<'a> {
    ifs: &'a IFS,
    max_depth: usize,
    // pairs of (depth, xform)
    stack: Vec<(usize, Mobius)>,
}

impl<'a> IFSDepthFirstIterator<'a> {
    fn new(ifs: &'a IFS, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(0, Mobius::IDENTITY)],
        }
    }
}

impl<'a> Iterator for IFSDepthFirstIterator<'a> {
    type Item = (usize, Mobius);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((depth, xform)) => {
                if depth < self.max_depth {
                    for next_xform in self.ifs.iter().cloned() {
                        self.stack.push((depth + 1, next_xform * xform));
                    }
                }
                Some((depth, xform))
            }
        }
    }
}

pub fn transform_tile(
    ifs: &IFS,
    tile: &ClineArcTile,
    min_depth: usize,
    max_depth: usize,
) -> Vec<ClineArcTile> {
    ifs.dfs(max_depth)
        .filter(|(depth, _)| *depth >= min_depth)
        .map(|(_, xform)| tile.transform(xform))
        .collect()
}
