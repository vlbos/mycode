/*
 * @lc app=leetcode id=1037 lang=rust
 *
 * [1037] Valid Boomerang
 */

// @lc code=start
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
       ( points[1][0]-points[0][0])* (points[2][1]-points[0][1])!=( points[2][0]-points[0][0])* (points[1][1]-points[0][1])
    }
}
// @lc code=end

