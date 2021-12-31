/*
 * @lc app=leetcode id=289 lang=rust
 *
 * [289] Game of Life
 */

// @lc code=start
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let b = board.clone();
        let m = b.len();
        let n = b[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut l = 0;
                if i > 0 {
                    l += b[i - 1][j];
                    if j > 0 {
                        l += b[i - 1][j - 1];
                    }
                    if j < n - 1 {
                        l += b[i - 1][j + 1];
                    }
                }
                if i < m - 1 {
                    l += b[i + 1][j];
                    if j > 0 {
                        l += b[i + 1][j - 1];
                    }
                    if j < n - 1 {
                        l += b[i + 1][j + 1];
                    }
                }
                if j > 0 {
                    l += b[i][j - 1];
                }
                if j < n - 1 {
                    l += b[i][j + 1];
                }

                if b[i][j] == 0 {
                    if l == 3 {
                        board[i][j] = 1;
                    }
                } else {
                    if l < 2 || l > 3 {
                        board[i][j] = 0;
                    }
                }
            }
        }
    }
}
// @lc code=end
