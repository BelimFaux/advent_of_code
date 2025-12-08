use std::mem::swap;

#[derive(Default, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    /// create a new union find structure with `size` sets of size 1
    pub fn new(size: usize) -> UnionFind {
        let parent = Vec::from_iter(0..size);
        let sizes = vec![1; size];
        UnionFind { parent, sizes }
    }

    /// add a new set with root node `x`
    pub fn make(&mut self, x: usize) {
        self.parent[x] = x;
        self.sizes[x] = 1;
    }

    /// find the root for node `x`
    /// uses path compression
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    /// balances the tree to reduce lookup time
    fn link(&mut self, mut x: usize, mut y: usize) {
        if self.sizes[x] < self.sizes[y] {
            swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.sizes[x] += self.sizes[y];
    }

    /// combine two trees
    pub fn union(&mut self, x: usize, y: usize) {
        let xroot = self.find(x);
        let yroot = self.find(y);
        if xroot != yroot {
            self.link(xroot, yroot);
        }
    }

    /// size of the set `x` is contained in
    pub fn size_of(&mut self, x: usize) -> usize {
        let xroot = self.find(x);
        self.sizes[xroot]
    }

    /// calculate the sizes of all sets
    pub fn sizes(&self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter(|(i, e)| &i == e)
            .map(|(_, e)| self.sizes[*e])
            .collect()
    }
}
