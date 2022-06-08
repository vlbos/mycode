/*
 * @lc app=leetcode id=132 lang=rust
 *
 * [132] Palindrome Partitioning II
 */

// @lc code=start
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let bs = s.as_bytes();
        let mut g = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                g[i][j] = (bs[i] == bs[j] && g[i + 1][j - 1]);
            }
        }
        let mut f = vec![i32::MAX; n];
        for i in 0..n {
            if g[0][i] {
                f[i] = 0;
                continue;
            }
            for j in 0..i {
                if g[j + 1][i] {
                    f[i] = f[i].min(f[j] + 1);
                }
            }
        }
        f[n - 1]
    }
}
// @lc code=end
