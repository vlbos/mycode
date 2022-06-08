/*
 * @lc app=leetcode id=1401 lang=rust
 *
 * [1401] Circle and Rectangle Overlapping
 */

// @lc code=start
impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let dx = if x1>x_center{x1-x_center}else if x2<x_center{x_center-x2}else{0};
        let dy = if y1>y_center{y1-y_center}else if y2<y_center{y_center-y2}else{0};
        dx*dx+dy*dy<=radius*radius
    }
}
// @lc code=end

