/*
 * @lc app=leetcode id=1779 lang=rust
 *
 * [1779] Find Nearest Point That Has the Same X or Y Coordinate
 */

// @lc code=start
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let mut ans = -1;
        for (i, n) in points.iter().enumerate() {
            if n[0] == x || n[1] == y {
                let v = (n[0] - x).abs() + (n[1] - y).abs();
                if v < min {
                    min = v;
                    ans = i as i32;
                }
            }
        }
        ans
    }
}
// @lc code=end
