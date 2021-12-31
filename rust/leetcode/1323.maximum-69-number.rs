/*
 * @lc app=leetcode id=1323 lang=rust
 *
 * [1323] Maximum 69 Number
 */

// @lc code=start
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut m =  0;
        let mut i = 1;
        while num > i {
            if num / i%10 == 6 {
                m = i;
            }
            i *= 10;
        }
        num+m*3
    }
}
// @lc code=end
