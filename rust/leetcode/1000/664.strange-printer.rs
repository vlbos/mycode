/*
 * @lc app=leetcode id=664 lang=rust
 *
 * [664] Strange Printer
 */

// @lc code=start
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![vec![0; n]; n];
        let bs = s.as_bytes();
        for i in (0..n).rev() {
            f[i][i] = 1;
            for j in i + 1..n {
                if bs[i] == bs[j] {
                    f[i][j] = f[i][j - 1];
                } else {
                    let mut min = i32::MAX;
                    for k in i..j {
                        min = min.min(f[i][k] + f[k + 1][j]);
                    }
                    f[i][j] = min;
                }
            }
        }
        f[0][n - 1]
    }
}
// @lc code=end
