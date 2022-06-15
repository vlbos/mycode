/*
 * @lc app=leetcode id=2169 lang=rust
 *
 * [2169] Count Operations to Obtain Zero
 */

// @lc code=start
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut ans = 0;
        let (mut num1, mut num2) = (num1, num2);
        while num1 > 0 && num2 > 0 {
            ans += num1/num2;
            num1%=num2;
            std::mem::swap(&mut num1,&mut num2);
        }
        ans
    }
}
// @lc code=end
