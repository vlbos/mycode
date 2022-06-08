/*
 * @lc app=leetcode id=1844 lang=rust
 *
 * [1844] Replace All Digits with Characters
 */

// @lc code=start
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut v  = s.chars().collect::<Vec<char>>();
        for i in (1..v.len()).step_by(2){
             v[i] =  (v[i-1] as u8+(v[i] as u8-'0' as u8)) as char;
        }
        v.iter().collect()
    }
}
// @lc code=end

