/*
 * @lc app=leetcode id=1557 lang=rust
 *
 * [1557] Minimum Number of Vertices to Reach All Nodes
 */

// @lc code=start
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let end_set=edges.iter().map(|x|x[1]).collect::<std::collections::HashSet<_>>();
        let ans = (0..n).collect::<Vec<i32>>();
        ans.iter().filter(|x|!end_set.contains(x)).cloned().collect()
    }
}
// @lc code=end

