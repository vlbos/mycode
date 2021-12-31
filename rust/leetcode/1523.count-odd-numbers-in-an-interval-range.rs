/*
 * @lc app=leetcode id=1523 lang=rust
 *
 * [1523] Count Odd Numbers in an Interval Range
 */

// @lc code=start
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high+1)>>1)-(low>>1)
    }
}
// @lc code=end

