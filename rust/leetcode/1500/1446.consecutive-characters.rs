/*
 * @lc app=leetcode id=1446 lang=rust
 *
 * [1446] Consecutive Characters
 */

// @lc code=start
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max = 0;
        let mut rc = ' ';
        let mut cnt = 0;
        for c in s.chars() {
            if rc != c {
                if cnt > max {
                    max = cnt;
                }
                rc = c;
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        if cnt > max {
            max = cnt;
        }
        max
    }
}
// @lc code=end
