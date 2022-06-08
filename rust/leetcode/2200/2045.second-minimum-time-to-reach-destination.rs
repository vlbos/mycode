/*
 * @lc app=leetcode id=2045 lang=rust
 *
 * [2045] Second Minimum Time to Reach Destination
 */

// @lc code=start
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![Vec::new(); n + 1];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut p = vec![vec![i32::MAX; 2]; n + 1];
        p[1][0] = 0;
        let mut q = std::collections::VecDeque::from([(1, 0)]);
        while p[n][1] == i32::MAX {
            let (cur, len) = q.pop_front().unwrap();
            for &next in &g[cur] {
                if len + 1 < p[next][0] {
                    p[next][0] = len + 1;
                    q.push_back((next, len + 1));
                } else if len + 1 > p[next][0] && len + 1 < p[next][1] {
                    p[next][1] = len + 1;
                    q.push_back((next, len + 1));
                }
            }
        }
        let mut ans = 0;
        for _ in 0..p[n][1] {
            if ans % (2 * change) >= change {
                ans += 2 * change - ans % (2 * change);
            }
            ans += time;
        }
        ans
    }
}
// @lc code=end
