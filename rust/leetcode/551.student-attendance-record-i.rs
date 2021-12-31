/*
 * @lc app=leetcode id=551 lang=rust
 *
 * [551] Student Attendance Record I
 */

// @lc code=start
impl Solution {
    pub fn check_record(s: String) -> bool {
         let mut a = 0;
        let mut l = 0;
        for c in s.chars() {
            if c == 'L' {
                l += 1;
                if l > 2 {
                    return false;
                }
            } else {
                l = 0;
                if c == 'A' {
                    a += 1;
                    if a > 1 {
                        return false;
                    }
                }
            }
        }
        a <= 1 && l <= 2
    }
}
// @lc code=end

