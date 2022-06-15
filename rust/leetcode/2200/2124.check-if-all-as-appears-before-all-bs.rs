/*
 * @lc app=leetcode id=2124 lang=rust
 *
 * [2124] Check if All A's Appears Before All B's
 */

// @lc code=start
impl Solution {
    pub fn check_string(s: String) -> bool {
        let a=s.rfind(|c|c=='a');
        let b=s.find(|c|c=='b');
        a.is_none() ||b.is_none()|| a.unwrap()<b.unwrap()
    }
}
// @lc code=end

