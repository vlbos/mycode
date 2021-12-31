/*
 * @lc app=leetcode id=794 lang=rust
 *
 * [794] Valid Tic-Tac-Toe State
 */

// @lc code=start
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut xcount = 0;
        let mut ocount = 0;
        for b in &board {
            for c in b.chars() {
                if c == 'X' {
                    xcount += 1;
                } else if c == 'O' {
                    ocount += 1;
                }
            }
        }
        let win = |board: &Vec<String>, p: char| -> bool {
            for i in 0..3 {
                if board[i].chars().all(|x| x == p)
                    || board.iter().all(|x| x.chars().nth(i).unwrap() == p)
                {
                    return true;
                }
            }
            board[0].chars().nth(0).unwrap() == p
                && board[1].chars().nth(1).unwrap() == p
                && board[2].chars().nth(2).unwrap() == p
                || board[0].chars().nth(2).unwrap() == p
                    && board[1].chars().nth(1).unwrap() == p
                    && board[2].chars().nth(0).unwrap() == p
        };
        !((ocount != xcount && ocount != xcount - 1)
            || (ocount != xcount - 1 && win(&board, 'X'))
            || (ocount != xcount && win(&board, 'O')))
    }
}
// @lc code=end
