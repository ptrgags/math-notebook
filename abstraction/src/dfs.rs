use std::{collections::HashSet, hash::Hash};

pub trait DFSTraversal<H, I>
where
    H: Eq,
    I: Copy,
{
    fn get_vertex_count(&self) -> usize;
    fn pick_start(&self, visited_set: &HashSet<H>) -> I;
    fn hash(&self, index: I) -> H;
    fn get_neighbors(&self, index: I) -> Vec<I>;
    fn order_neighbors(&self, neighbors: &[I]) -> Vec<I>;
}

pub struct DFS<'a, H, I>
where
    I: Copy,
{
    traversal: Box<dyn DFSTraversal<H, I> + 'a>,
}

impl<'a, H, I> DFS<'a, H, I>
where
    H: Eq + Hash,
    I: Copy,
{
    pub fn new(traversal: impl DFSTraversal<H, I> + 'a) -> Self {
        Self {
            traversal: Box::new(traversal),
        }
    }

    pub fn dfs_forest(&self, callback: fn(&[I])) {
        let mut visited = HashSet::new();
        let vertex_count = self.traversal.get_vertex_count();

        while visited.len() < vertex_count {
            let start_index = self.traversal.pick_start(&visited);
            self.dfs_tree(start_index, callback, &mut visited);
        }
    }

    pub fn dfs_tree(&self, start_index: I, callback: fn(&[I]), visited: &mut HashSet<H>) {
        let mut stack = vec![(start_index, vec![start_index])];
        while let Some((current_index, current_path)) = stack.pop() {
            let hash = self.traversal.hash(current_index);
            if (visited.contains(&hash)) {
                continue;
            }

            visited.insert(hash);
            callback(&current_path);

            let unvisited_neighbors: Vec<I> = self
                .traversal
                .get_neighbors(current_index)
                .into_iter()
                .filter(|index| !visited.contains(&self.traversal.hash(*index)))
                .collect();

            let ordered_neighbors = self.traversal.order_neighbors(&unvisited_neighbors);

            for neighbor in ordered_neighbors {
                let mut path = current_path.clone();
                path.push(neighbor);
                stack.push((neighbor, path));
            }
        }
    }
}
