/*
 * @lc app=leetcode id=2119 lang=rust
 *
 * [2119] A Number After a Double Reversal
 */

// @lc code=start
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        if num < 10 {
            return true;
        }
        let reverse = |num: i32| -> i32 {
            let mut tmp = num;
            let mut reversed_num = 0;
            while tmp > 0 {
                reversed_num = reversed_num * 10 + tmp % 10;
                tmp /= 10;
            }
            reversed_num
        };
        let reversed_num = reverse(num);
        reverse(reversed_num) == num
    }
}
// @lc code=end
