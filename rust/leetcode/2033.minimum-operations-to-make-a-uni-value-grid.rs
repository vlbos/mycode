/*
 * @lc app=leetcode id=2033 lang=rust
 *
 * [2033] Minimum Operations to Make a Uni-Value Grid
 */

// @lc code=start
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut g = grid.into_iter().flatten().collect::<Vec<i32>>();
        g.sort();
        let m = g[g.len() / 2];
        let mut ans = 0;
        for &v in &g {
            let d = (m - v).abs();
            if d % x > 0 {
                return -1;
            }
            ans += d / x;
        }
        ans
    }
}
// @lc code=end
