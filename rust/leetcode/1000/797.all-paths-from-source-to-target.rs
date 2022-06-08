/*
 * @lc app=leetcode id=797 lang=rust
 *
 * [797] All Paths From Source to Target
 */

// @lc code=start
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut tmp = Vec::new();
        fn dfs(
            graph: &Vec<Vec<i32>>,
            ans: &mut Vec<Vec<i32>>,
            tmp: &mut Vec<i32>,
            i: usize,
            n: usize,
        ) {
            if i == n {
                ans.push(tmp.clone());
                return;
            }
            for g in &graph[i] {
                tmp.push(*g);
                dfs(graph, ans, tmp, *g as usize, graph.len() - 1);
                tmp.pop();
            }
        }
        tmp.push(0);
        dfs(&graph, &mut ans, &mut tmp, 0, graph.len() - 1);
        ans
    }
}
// @lc code=end
