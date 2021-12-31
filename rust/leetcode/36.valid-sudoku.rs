/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut flags = vec![0; 9];
        for i in 0..board.len() {
            flags = vec![0; 9];
            for j in 0..board[i].len() {
                if board[i][j].is_ascii_digit() {
                    let index = (board[i][j] as u8 - '1' as u8) as usize;
                    if flags[index] != 0 {
                        return false;
                    }
                    flags[index] = 1;
                }
            }
            flags = vec![0; 9];
            for j in 0..board[i].len() {
                if board[j][i].is_ascii_digit() {
                    let index = (board[j][i] as u8 - '1' as u8) as usize;
                    if flags[index] != 0 {
                        return false;
                    }
                    flags[index] = 1;
                }
            }
        }

        for k in (0..=6).step_by(3) {
            for l in (0..=6).step_by(3) {
                flags = vec![0; 9];
                for i in k..k + 3 {
                    for j in l..l + 3 {
                        if board[i][j].is_ascii_digit() {
                            let index = (board[i][j] as u8 - '1' as u8) as usize;
                            if flags[index] != 0 {
                                return false;
                            }
                            flags[index] = 1;
                        }
                    }
                }
            }
        }

        true
    }
}
// @lc code=end
