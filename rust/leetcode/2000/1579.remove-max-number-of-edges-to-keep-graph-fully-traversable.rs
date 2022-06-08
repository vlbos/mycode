/*
 * @lc app=leetcode id=1579 lang=rust
 *
 * [1579] Remove Max Number of Edges to Keep Graph Fully Traversable
 */

// @lc code=start
struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
    set_count: i32,
}
impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n as usize],
            set_count: n,
        }
    }
    fn find_set(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] == x {
            return x;
        }
        self.parent[x as usize] = self.find_set(self.parent[x as usize]);
        self.parent[x as usize]
    }
    fn unite(&mut self, x: i32, y: i32) -> bool {
        let (x, y) = (self.find_set(x), self.find_set(y));
        if x == y {
            return false;
        }
        let (x, y) = if self.size[x as usize] < self.size[y as usize] {
            (y, x)
        } else {
            (x, y)
        };
        self.parent[y as usize] = x;
        self.size[x as usize] += self.size[y as usize];
        self.set_count -= 1;
        true
    }
}
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let (mut ufa, mut ufb) = (UnionFind::new(n), UnionFind::new(n));
        let mut ans = 0;
        let edges: Vec<(i32,i32,i32)> = edges
            .into_iter()
            .map(|x| (x[0], x[1] - 1, x[2] - 1))
            .collect();
        for &(t, u, v) in &edges {
            if t == 3 {
                if !ufa.unite(u, v) {
                    ans += 1;
                } else {
                    ufb.unite(u, v);
                }
            }
        }
        for &(t, u, v) in &edges {
            if t == 1 {
                if !ufa.unite(u, v) {
                    ans += 1;
                }
            } else if t == 2 {
                if !ufb.unite(u, v) {
                    ans += 1;
                }
            }
        }
        if ufa.set_count != 1 || ufb.set_count != 1 {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
