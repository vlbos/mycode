/*
 * @lc app=leetcode id=1015 lang=rust
 *
 * [1015] Smallest Integer Divisible by K
 */

// @lc code=start
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        let mut tmp = 1;
        let mut len = 1;
        while tmp % k != 0 {
            tmp = (tmp % k) * 10 + 1;
            len += 1;
        }
        len
    }
}
// @lc code=end
