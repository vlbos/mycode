/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        for (i, _) in haystack.chars().enumerate() {
            if i + needle.len() > haystack.len() {
                return -1;
            }
            if &haystack[i..i + needle.len()] == &needle {
                return i as i32;
            }
        }
        -1
    }
}
// @lc code=end

