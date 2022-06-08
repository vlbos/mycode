/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans = vec![vec![0;n];n];
        let mut i = 0;
        let mut j = 0;

        let mut d = 0;
        let mut offset= 0;
        for k in 0..n * n {
            ans[i][j]=k as i32+1;
            offset = (d+1)/4;
            let dd = d%4;
            match dd {
                0 => {
                    if j == n - 1-offset {
                        i += 1;
                        d += 1;
                    } else {
                        j += 1;
                    }
                }
                1 => {
                    if i == n - 1-offset {
                        j -= 1;
                        d += 1;
                    } else {
                        i += 1;
                    }
                }
                2 => {
                    if j == offset {
                        i -= 1;
                        d += 1;
                    } else {
                        j -= 1;
                    }
                }
                _ => {
                    if i == offset {
                        j += 1;
                        d += 1;
                    } else {
                        i -= 1;
                    }
                }
            };
        }

        ans
    }
}
// @lc code=end

