/*
 * @lc app=leetcode id=1556 lang=rust
 *
 * [1556] Thousand Separator
 */

// @lc code=start
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n.to_string().len() < 4 {
            return n.to_string();
        }
        let mut ns = n.to_string().chars().collect::<Vec<char>>();
        let mut i = ns.len() - 3;
        while i > 0 {
            if i > 0 {
                ns.insert(i, '.');
            }
            if i > 3 {
                i -= 3;
            } else {
                break;
            }
        }
        ns.iter().collect()
    }
}
// @lc code=end
