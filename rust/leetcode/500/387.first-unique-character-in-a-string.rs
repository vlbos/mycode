/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */

// @lc code=start
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut index = Vec::<i32>::new();
        let mut chars = Vec::<char>::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(j) = chars.iter().position(|&v| v == c) {
                index[j] = -1;
            } else {
                chars.push(c);
                index.push(i as i32);
            }
        }
        for k in index {
            if k != -1 {
                return k;
            }
        }
        -1
    }
}
// @lc code=end

