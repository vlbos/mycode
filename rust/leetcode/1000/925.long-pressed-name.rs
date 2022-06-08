/*
 * @lc app=leetcode id=925 lang=rust
 *
 * [925] Long Pressed Name
 */

// @lc code=start
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
         let mut i = 0;
        let mut j = 0;
        while j < typed.len() {
            if i < name.len() && name.chars().nth(i) == typed.chars().nth(j) {
                i += 1;
                j += 1;
            } else if j > 0 && typed.chars().nth(j - 1) == typed.chars().nth(j) {
                j += 1;
            } else {
                return false;
            }
        }
        i == name.len()
    }
}
// @lc code=end

