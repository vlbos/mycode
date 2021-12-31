/*
 * @lc app=leetcode id=942 lang=rust
 *
 * [942] DI String Match
 */

// @lc code=start
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
         let mut r = vec![0; s.len() + 1];
        let mut lo = 0;
        let mut hi = s.len() as i32;
        for (i, c) in s.chars().enumerate() {
            if c == 'I' {
                r[i] = lo;
                lo += 1;
            } else {
                r[i] = hi;
                hi -= 1;
            }
        }
        r[s.len()] = lo;
        r.to_vec()
    }
}
// @lc code=end

