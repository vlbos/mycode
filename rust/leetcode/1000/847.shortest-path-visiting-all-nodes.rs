/*
 * @lc app=leetcode id=847 lang=rust
 *
 * [847] Shortest Path Visiting All Nodes
 */

// @lc code=start
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut q: std::collections::VecDeque<(usize, usize, i32)> =
            (0..n).map(|x| (x, 1 << x, 0)).collect();
        let mut seen: std::collections::HashSet<(usize, usize)> =
            (0..n).map(|x| (x, 1 << x)).collect();

        let mut ans = 0;
        while let Some((u, mask, dist)) = q.pop_front() {
            if mask == (1 << n) - 1 {
                ans = dist;
                break;
            }
            for &v in &graph[u] {
                let v = v as usize;
                let v_mask = mask | (1 << v );
                if !seen.contains(&(v, v_mask)) {
                    q.push_back((v, v_mask, dist + 1));
                    seen.insert((v, v_mask));
                }
            }
        }
        ans 
    }
}
// @lc code=end
