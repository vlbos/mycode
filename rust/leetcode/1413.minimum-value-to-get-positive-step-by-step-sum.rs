/*
 * @lc app=leetcode id=1413 lang=rust
 *
 * [1413] Minimum Value to Get Positive Step by Step Sum
 */

// @lc code=start
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut nsum = 0;
        let mut psum = 0;
        let mut sv = 0;
        for n in &nums {
            if *n < 0 {
                nsum += *n;
            } else {
                psum += *n;
            }
            let m = nsum.abs() - psum + 1;
            if psum <= nsum.abs() && m > sv {
                sv = m;
            }
        }
        if sv == 0 {
            sv = 1;
        }
        sv
    }
}
// @lc code=end
