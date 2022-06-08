/*
 * @lc app=leetcode id=1857 lang=rust
 *
 * [1857] Largest Color Value in a Directed Graph
 */

// @lc code=start
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut g = vec![Vec::new(); n];
        let mut indeg = vec![0; n];
        for edge in &edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            indeg[v] += 1;
            g[u].push(v);
        }
        let mut f = vec![vec![0; 26]; n];
        let bc = colors.as_bytes();
        let mut found = 0;
        let mut q = std::collections::VecDeque::from(
            indeg
                .iter()
                .enumerate()
                .filter(|x| *x.1 == 0)
                .map(|x| x.0)
                .collect::<Vec<usize>>(),
        );
        while let Some(u) = q.pop_front() {
            found += 1;
            f[u][(bc[u] - b'a') as usize] += 1;
            for &v in &g[u] {
                indeg[v] -= 1;
                for c in 0..26 {
                    f[v][c] = f[v][c].max(f[u][c]);
                }
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if found != n {
            return -1;
        }
        f.into_iter()
            .map(|x| x.into_iter().max().unwrap())
            .max()
            .unwrap()
    }
}
// @lc code=end
