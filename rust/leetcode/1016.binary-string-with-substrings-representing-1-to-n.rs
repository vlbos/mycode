/*
 * @lc app=leetcode id=1016 lang=rust
 *
 * [1016] Binary String With Substrings Representing 1 To N
 */

// @lc code=start
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
       for i in 1..=n {
            if s.find(&format!("{:b}",i)).is_none() {
                return false;
            }
        }
        true
    }
}
// @lc code=end
