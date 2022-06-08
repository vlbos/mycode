/*
 * @lc app=leetcode id=1422 lang=rust
 *
 * [1422] Maximum Score After Splitting a String
 */

// @lc code=start
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let oc = s.chars().filter(|c| *c == '1').count();
        if oc == 0 || oc == s.len() {
            return s.len() as i32 - 1;
        }
        let mut m = 0;
        let mut lc = 0;
        let mut rc = 0;
        for i in 0..s.len() - 1 {
            if s.chars().nth(i).unwrap() == '0' {
                lc += 1;
            } else {
                rc += 1;
            }
            let cnt = lc + oc - rc;
            if cnt > m {
                m = cnt;
            }
        }
        m as i32
    }
}
// @lc code=end
