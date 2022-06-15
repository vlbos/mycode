/*
 * @lc app=leetcode id=2042 lang=rust
 *
 * [2042] Check if Numbers Are Ascending in a Sentence
 */

// @lc code=start
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut pre = -1;
        for v in s.split_ascii_whitespace() {
            if let Ok(num) = v.parse::<i32>() {
                if pre >= num {
                    return false;
                }
                pre = num;
            }
        }
        true
    }
}
// @lc code=end
