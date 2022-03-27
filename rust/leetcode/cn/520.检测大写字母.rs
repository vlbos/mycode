/*
 * @lc app=leetcode.cn id=520 lang=rust
 *
 * [520] 检测大写字母
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut r = 0;
        for c in word.chars() {
            if c.is_ascii_uppercase() {
                r += 1;
            }
        }
        r == 0 || (r == 1 && word.chars().next().unwrap().is_ascii_uppercase()) || r == word.len()
    }
}
// @lc code=end
