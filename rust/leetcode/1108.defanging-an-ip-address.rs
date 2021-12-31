/*
 * @lc app=leetcode id=1108 lang=rust
 *
 * [1108] Defanging an IP Address
 */

// @lc code=start
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut a = address;
        let mut v = Vec::new();
        for (i, c) in a.chars().enumerate() {
            if c == '.' {
                v.push(i);
            }
        }
        for i in v.iter().rev() {
            a.insert(*i + 1, ']');
            a.insert(*i, '[');
        }
        a
    }
}
// @lc code=end
