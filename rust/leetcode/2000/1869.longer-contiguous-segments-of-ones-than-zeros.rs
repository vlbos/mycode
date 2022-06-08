/*
 * @lc app=leetcode id=1869 lang=rust
 *
 * [1869] Longer Contiguous Segments of Ones than Zeros
 */

// @lc code=start
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut a = vec![0; 2];
        let mut m = vec![0; 2];
        let mut p = ' ';
        for c in s.chars() {
            let i = (c as u8 - '0' as u8) as usize;
            if c != p {
                m[i] = m[i].max(a[i]);
                a[i] = 1;
                p = c;
            } else {
                a[i] += 1;
            }
        }
        for i in 0..2 {
            m[i] = m[i].max(a[i]);
        }
        m[0] < m[1]
    }
}
// @lc code=end
