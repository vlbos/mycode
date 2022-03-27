/*
 * @lc app=leetcode id=1895 lang=rust
 *
 * [1895] Largest Magic Square
 */

// @lc code=start
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
         let (m, n) = (grid.len(), grid[0].len());
        let (mut rs, mut cs) = (vec![vec![0;n];m], vec![vec![0;n];m]);
        for i in 0..m {
            rs[i][0] = grid[i][0];
            for j in 1..n {
                rs[i][j] = rs[i][j - 1] + grid[i][j];
            }
        }
        for j in 0..n {
            cs[0][j] = grid[0][j];
            for i in 1..m {
                cs[i][j] = cs[i - 1][j] + grid[i][j];
            }
        }
        for e in (2..=m.min(n)).rev() {
            for i in 0..=m - e {
                for j in 0..=n - e {
                    let ss = rs[i][j + e - 1] - if j > 0 { rs[i][j - 1] } else { 0 };
                    let mut check = false;
                    for k in i + 1..i + e {
                        if rs[k][j + e - 1] - if j > 0 { rs[k][j - 1] } else { 0 } != ss {
                            check = true;
                            break;
                        }
                    }
                    if check {
                        continue;
                    }
                    for k in j..j + e {
                        if cs[i + e - 1][k] - if i > 0 { cs[i - 1][k] } else { 0 } != ss {
                            check = true;
                            break;
                        }
                    }
                    if check {
                        continue;
                    }
                    let (mut d1, mut d2) = (0, 0);
                    for k in 0..e {
                        d1 += grid[i + k][j + k];
                        d2 += grid[i + k][j + e - k - 1];
                    }
                    if d1 == ss && d2 == ss {
                        return e as i32;
                    }
                }
            }
        }
        1
    }
}
// @lc code=end
