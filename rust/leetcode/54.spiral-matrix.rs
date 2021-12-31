/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut d = 0;
        let mut offset= 0;
        for _ in 0..m * n {
            ans.push(matrix[i][j]);
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
                    if i == m - 1-offset {
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
