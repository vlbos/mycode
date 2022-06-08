/*
 * @lc app=leetcode id=802 lang=rust
 *
 * [802] Find Eventual Safe States
 */

// @lc code=start
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut color = vec![0; n];
        fn safe(color: &mut Vec<i32>, graph: &Vec<Vec<i32>>, i: usize) -> bool {
            if color[i] > 0 {
                return color[i] == 2;
            }
            color[i] = 1;
            for g in &graph[i] {
                if !safe(color, graph, *g as usize) {
                    return false;
                }
            }
            color[i] = 2;
            true
        }
        let mut ans = Vec::new();
        for i in 0..n {
            if safe(&mut color, &graph, i) {
                ans.push(i as i32);
            }
        }
        ans
    }
}
// @lc code=end
