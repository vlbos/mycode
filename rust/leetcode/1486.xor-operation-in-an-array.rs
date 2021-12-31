/*
 * @lc app=leetcode id=1486 lang=rust
 *
 * [1486] XOR Operation in an Array
 */

// @lc code=start
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n{
            ans ^= start+2*i;
        }
        ans
    }
}
// @lc code=end

