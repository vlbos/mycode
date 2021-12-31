/*
 * @lc app=leetcode id=1784 lang=rust
 *
 * [1784] Check if Binary String Has at Most One Segment of Ones
 */

// @lc code=start
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let l = s.find("1");
        let r = s.rfind("1");
        let ones =  s.chars().filter(|c|*c=='1').count();
        (r.unwrap()-l.unwrap()+1)==ones
    }
}
// @lc code=end

