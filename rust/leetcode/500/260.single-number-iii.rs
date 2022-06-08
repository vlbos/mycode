/*
 * @lc app=leetcode id=260 lang=rust
 *
 * [260] Single Number III
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = 0;
        for n in &nums {
            ret ^= *n;
        }
        let mut div = 1;
        while div & ret == 0 {
            div <<= 1;
        }
        let mut a = 0;
        let mut b = 0;
        for n in &nums {
            if div & *n == 0 {
                a ^= *n;
            } else {
                b ^= *n;
            }
        }
        vec![a, b]
    }
}
// @lc code=end
