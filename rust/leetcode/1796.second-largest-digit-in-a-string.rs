/*
 * @lc app=leetcode id=1796 lang=rust
 *
 * [1796] Second Largest Digit in a String
 */

// @lc code=start
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut ss = vec![-1;2];
        for c in s.chars() {
            if c.is_ascii_digit() {
                let d = (c as u8 - '0' as u8) as i32;
                if ss[0]==-1 {
                    ss[0]=d;
                }else if ss[0]<d{
                    ss[1]=ss[0];
                    ss[0]=d;
                }else if ss[0]>d && ss[1]<d{
                    ss[1]=d;
                }
            }
        }
        ss[1]
    }
}
// @lc code=end
