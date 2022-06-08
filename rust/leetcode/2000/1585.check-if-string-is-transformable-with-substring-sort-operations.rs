/*
 * @lc app=leetcode id=1585 lang=rust
 *
 * [1585] Check If String Is Transformable With Substring Sort Operations
 */

// @lc code=start
impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let mut pos = vec![Vec::new(); 10];
        for (i, b) in s.bytes().enumerate().rev() {
            pos[(b - b'0') as usize].push(i);
        }
        for b in t.bytes() {
            let digit = (b - b'0') as usize;
            if pos[digit].is_empty() {
                return false;
            }
            for j in 0..digit {
                if !pos[j].is_empty() && *pos[j].last().unwrap() < *pos[digit].last().unwrap() {
                    return false;
                }
            }
            pos[digit].pop();
        }
        true
    }
}
// @lc code=end
