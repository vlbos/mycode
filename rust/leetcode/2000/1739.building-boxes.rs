/*
 * @lc app=leetcode id=1739 lang=rust
 *
 * [1739] Building Boxes
 */

// @lc code=start
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
         let (mut left, mut right) = (1, n);
        while left < right {
            let mid = ((left + right) / 2) as f64;
            if mid * (mid + 1.0) * (mid + 2.0) / 6.0 < n as f64 {
                left = mid as i32 + 1;
            } else {
                right = mid as i32;
            }
        }
        left -= 1;
        let mid = left as i64;
        let cell = mid * (mid + 1) * (mid + 2) / 6;
        let area = (mid + 1) * mid / 2;
        let target = n as i64 - cell;
        let (mut left, mut right) = (0, target);
        while left < right {
            let mid = (left + right) / 2;
            if mid * (mid + 1) / 2 < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        (area + left) as _
    }
}
// @lc code=end
