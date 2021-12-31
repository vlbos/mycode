/*
 * @lc app=leetcode id=201 lang=rust
 *
 * [201] Bitwise AND of Numbers Range
 */

// @lc code=start
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut ans = right;
        while left <ans {
                ans = ans&(ans-1);
        }
        ans
    }
}
// @lc code=end
