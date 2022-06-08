/*
 * @lc app=leetcode id=1632 lang=rust
 *
 * [1632] Rank Transform of a Matrix
 */

// @lc code=start
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }
    fn connect(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        let (px, py) = if self.size[px] > self.size[py] {
            (py, px)
        } else {
            (px, py)
        };
        self.parent[px] = py;
        self.size[py] += self.size[px];
    }
}
impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
       let (m, n) = (matrix.len(), matrix[0].len());
        let mn = m * n;
        let mut uf = UnionFind::new(mn);
        use std::collections::HashMap;
        for i in 0..m {
            let mut mp = HashMap::new();
            for j in 0..n {
                mp.entry(matrix[i][j]).or_insert(Vec::new()).push(i * n + j);
            }
            for v in mp.values() {
                for w in v.windows(2) {
                    uf.connect(w[0], w[1]);
                }
            }
        }
        for j in 0..n {
            let mut mp = HashMap::new();
            for i in 0..m {
                mp.entry(matrix[i][j]).or_insert(Vec::new()).push(i * n + j);
            }
            for v in mp.values() {
                for w in v.windows(2) {
                    uf.connect(w[0], w[1]);
                }
            }
        }
        let mut adj = vec![Vec::new(); mn];
        let mut indegrees = vec![0; mn];

        for i in 0..m {
            let mut v = vec![(0,0); n];
            for j in 0..n {
                v[j] = (matrix[i][j], j);
            }
            v.sort();
            for w in v.windows(2) {
                if w[0].0 != w[1].0 {
                    let (x, y) = (uf.find(i * n + w[0].1), uf.find(i * n + w[1].1));
                    adj[x].push(y);
                    indegrees[y] += 1;
                }
            }
        }
        for j in 0..n {
               let mut v = vec![(0,0); m];
            for i in 0..m {
                v[i] = (matrix[i][j], i);
            }
            v.sort();
            for w in v.windows(2) {
                if w[0].0 != w[1].0 {
                    let (x, y) = (uf.find(j + w[0].1 * n), uf.find(j + w[1].1 * n));
                    adj[x].push(y);
                    indegrees[y] += 1;
                }
            }
        }

        let mut ans = vec![1; mn];
        let mut q = std::collections::VecDeque::new();
        for i in 0..mn {
            if uf.find(i) == i && indegrees[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                ans[v] = ans[v].max(ans[u] + 1);
                indegrees[v] -= 1;
                if indegrees[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        let mut result = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                result[i][j] = ans[uf.find(i * n + j)];
            }
        }
        result
    }
}
// @lc code=end
