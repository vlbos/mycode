/*
 * @lc app=leetcode id=2039 lang=rust
 *
 * [2039] The Time When the Network Becomes Idle
 */

// @lc code=start
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut vis = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0));
        vis[0] = true;
        let mut ans = 0;
        while let Some((c, time)) = q.pop_front() {
            if c != 0 {
                let cost = ((2 * time - 1) / patience[c]) * patience[c] + 2 * time;
                ans = ans.max(cost);
            }
            for &next in &g[c] {
                if !vis[next] {
                    vis[next] = true;
                    q.push_back((next, time + 1));
                }
            }
        }
        ans + 1
    }
}
// @lc code=end
