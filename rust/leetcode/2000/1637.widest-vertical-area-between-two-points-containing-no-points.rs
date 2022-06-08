/*
 * @lc app=leetcode id=1637 lang=rust
 *
 * [1637] Widest Vertical Area Between Two Points Containing No Points
 */

// @lc code=start
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|x| x[0]);
        points.windows(2).map(|x| x[1][0] - x[0][0]).max().unwrap()
    }
}
// @lc code=end
