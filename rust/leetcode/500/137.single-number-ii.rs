/*
 * @lc app=leetcode id=137 lang=rust
 *
 * [137] Single Number II
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut ans = 0;
        for i in 0..32 {
            total = 0;
            for n in &nums {
                total += ((*n)>>i) & 1 ;
            }
            if total % 3 == 1 {
                ans |= (1 << i);
            }
        }
        ans
    }
}
// @lc code=end
