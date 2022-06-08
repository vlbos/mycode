/*
 * @lc app=leetcode id=1668 lang=rust
 *
 * [1668] Maximum Repeating Substring
 */

// @lc code=start
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let v = sequence.matches(&word).collect::<Vec<&str>>();
        if v.len() < 2 {
            return v.len() as i32;
        }
        for i in (1..=v.len()).rev() {
            if sequence.contains(&word.repeat(i)) {
                return i as i32;
            }
        }
        0
    }
}
// @lc code=end
