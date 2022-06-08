/*
 * @lc app=leetcode id=1761 lang=rust
 *
 * [1761] Minimum Degree of a Connected Trio in a Graph
 */

// @lc code=start
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut d = vec![std::collections::HashSet::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize - 1, e[1] as usize - 1);
            d[u].insert(v);
            d[v].insert(u);
        }
        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize - 1, e[1] as usize - 1);
            if d[u].len() < d[v].len() || d[u].len() == d[v].len() && u < v {
                adj[u].push(v);
            } else {
                adj[v].push(u);
            }
        }
        let mut ans = i32::MAX;
        for u in 0..n {
            for &v in &adj[u] {
                for &w in &adj[v] {
                    if d[u].contains(&w) {
                        ans = ans.min((d[u].len() + d[v].len() + d[w].len()) as i32 - 6);
                    }
                }
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
