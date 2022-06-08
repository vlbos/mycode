/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (m, n) = (s.len(), p.len());
        let (bs, bp) = (s.as_bytes(), p.as_bytes());
        let matches = |i: usize, j: usize| -> bool {
            if i == 0 {
                return false;
            }
            if bp[j - 1] == b'.' {
                return true;
            }
            bs[i - 1] == bp[j - 1]
        };
        let mut f = vec![vec![false; n + 1]; m + 1];
        f[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                if bp[j - 1] == b'*' {
                    f[i][j] = f[i][j - 2];
                    if matches(i, j - 1) {
                        f[i][j] |= f[i - 1][j];
                    }
                } else {
                    if matches(i, j) {
                        f[i][j] |= f[i - 1][j - 1];
                    }
                }
            }
        }
        f[m][n]
    }
}
// @lc code=end
