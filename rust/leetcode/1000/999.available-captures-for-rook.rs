/*
 * @lc app=leetcode id=999 lang=rust
 *
 * [999] Available Captures for Rook
 */

// @lc code=start
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut r = 0;
        let mut i = board.len();
        let mut j = board.len();
        for rx in 0..board.len() {
            for ry in 0..board.len() {
                if board[rx][ry] == 'R' {
                    i = rx;
                    j = ry;
                    break;
                }
            }
        }
        let mut k = 0;
        if i > 0 {
            k = i - 1;
            while k > 0 && board[k][j] == '.' {
                k -= 1;
            }
            if board[k][j] == 'p' {
                r += 1;
            }
        }
        if i < board.len() - 1 {
            k = i + 1;
            while k < board.len() - 1 && board[k][j] == '.' {
                k += 1;
            }
            if board[k][j] == 'p' {
                r += 1;
            }
        }
        if j > 0 {
            k = j - 1;
            while k > 0 && board[i][k] == '.' {
                k -= 1;
            }
            if board[i][k] == 'p' {
                r += 1;
            }
        }
        if j < board.len() - 1 {
            k = j + 1;
            while k < board.len() - 1 && board[i][k] == '.' {
                k += 1;
            }
            if board[i][k] == 'p' {
                r += 1;
            }
        }
        r
    }
}
// @lc code=end
