/*
 * @lc app=leetcode id=1881 lang=rust
 *
 * [1881] Maximum Value after Insertion
 */

// @lc code=start
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let nb = n.as_bytes();
        let mut index = nb.len();
        let xb = x as u8 + b'0';
        if nb[0] == b'-' {
            if let Some(i) = nb.iter().position(|y| *y > xb) {
                index = i;
            }
        } else {
            if let Some(i) = nb.iter().position(|y| *y < xb) {
                index = i;
            }
        }
        let mut n = n;
        n.insert(index, xb as char);
        n
    }
}
// @lc code=end
