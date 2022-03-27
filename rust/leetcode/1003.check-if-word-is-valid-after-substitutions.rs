/*
 * @lc app=leetcode id=1003 lang=rust
 *
 * [1003] Check If Word Is Valid After Substitutions
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut s = s;
        let mut n  = s.len();
        while !s.is_empty(){
            n  = s.len();
            s=s.replace("abc","");
            if n==s.len(){
            break;
            }

        }
        s.is_empty()
    }
}
// @lc code=end

