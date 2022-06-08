/*
 * @lc app=leetcode id=1374 lang=rust
 *
 * [1374] Generate a String With Characters That Have Odd Counts
 */

// @lc code=start
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n < 2 {
            return "a".to_string();
        }
        if n % 2 == 0 {
            "a".repeat((n - 1) as usize) + "b"
        } else {
            "a".repeat(n as usize)
        }
    }
}
// @lc code=end
