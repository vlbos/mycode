/*
 * @lc app=leetcode id=1976 lang=rust
 *
 * [1976] Number of Ways to Arrive at Destination
 */

// @lc code=start
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dist = vec![vec![i64::MAX / 2; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for r in &roads {
            let (x, y, t) = (r[0] as usize, r[1] as usize, r[2] as i64);
            dist[x][y] = t;
            dist[y][x] = t;
        }
        let mut used = vec![false; n];
        for _ in 0..n {
            let mut u = n;
            for i in 0..n {
                if !used[i] && (u == n || dist[0][i] < dist[0][u]) {
                    u = i;
                }
            }
            used[u] = true;
            for i in 0..n {
                dist[0][i] = dist[0][i].min(dist[0][u] + dist[u][i]);
            }
        }
        let mut g = vec![Vec::new(); n];
        for r in &roads {
            let (x, y, t) = (r[0] as usize, r[1] as usize, r[2] as i64);
            if dist[0][y] - dist[0][x] == t {
                g[x].push(y);
            } else if dist[0][x] - dist[0][y] == t {
                g[y].push(x);
            }
        }
        let mut f = vec![-1; n];
        fn dfs(f: &mut Vec<i32>, g: &Vec<Vec<usize>>, u: usize) -> i32 {
            if u == f.len() - 1 {
                return 1;
            }
            if f[u] != -1 {
                return f[u];
            }
            f[u] = 0;
            for &v in &g[u] {
                f[u] += dfs(f, g, v);
                if f[u] >= 1000_000_007 {
                    f[u] -= 1000_000_007;
                }
            }
            f[u]
        }
        dfs(&mut f, &g, 0)
    }
}
// @lc code=end
