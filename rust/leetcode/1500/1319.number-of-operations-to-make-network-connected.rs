/*
 * @lc app=leetcode id=1319 lang=rust
 *
 * [1319] Number of Operations to Make Network Connected
 */

// @lc code=start
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() < n - 1 {
            return -1;
        }
        let mut edges = vec![Vec::new(); n];
        for c in &connections {
            edges[c[0] as usize].push(c[1] as usize);
            edges[c[1] as usize].push(c[0] as usize);
        }
        let mut used = vec![false; n];
        fn dfs(edges: &Vec<Vec<usize>>, used: &mut Vec<bool>, i: usize) {
            used[i] = true;
            for &j in &edges[i] {
                if !used[j] {
                    dfs(edges, used, j);
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            if !used[i] {
                ans += 1;
                dfs(&edges, &mut used, i);
            }
        }
        ans-1
    }
}
// @lc code=end
