/*
 * @lc app=leetcode id=419 lang=rust
 *
 * [419] Battleships in a Board
 */

// @lc code=start
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut cnt = 0;
        let mut m = board.len();
        let mut n = board[0].len();
        let mut vis = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if vis[i][j] {
                    continue;
                }
                if board[i][j] == 'X' {
                    let mut k = i + 1;
                    while k < m {
                        if !vis[k][j] && board[k][j] == 'X' {
                            vis[k][j] = true;
                        } else {
                            break;
                        }
                        k += 1;
                    }
                    let mut k = j + 1;
                    while k < n {
                        if !vis[i][k] && board[i][k] == 'X' {
                            vis[i][k] = true;
                        } else {
                            break;
                        }
                        k += 1;
                    }
                    vis[i][j] = true;
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
// @lc code=end
