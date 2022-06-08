/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 */

// @lc code=start
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut line = vec![0; 9];
        let mut column = vec![0; 9];
        let mut block = vec![vec![0; 3]; 3];
        let mut spaces = Vec::new();
        let mut valid = false;
        fn dfs(
            pos: usize,
            spaces: &Vec<Vec<usize>>,
            line: &mut Vec<i32>,
            column: &mut Vec<i32>,
            block: &mut Vec<Vec<i32>>,
            board: &mut Vec<Vec<char>>,
            valid: &mut bool,
        ) {
            let flip = |i: usize,
                        j: usize,
                        digit: i32,
                        line: &mut Vec<i32>,
                        column: &mut Vec<i32>,
                        block: &mut Vec<Vec<i32>>| {
                line[i] ^= (1 << digit);
                column[j] ^= (1 << digit);
                block[i / 3][j / 3] ^= (1 << digit);
            };
            if pos == spaces.len() {
                *valid = true;
                return;
            }
            let (i, j) = (spaces[pos][0], spaces[pos][1]);
            let mut mask = !(line[i] | column[j] | block[i / 3][j / 3]) & 0x1ff;
            while mask != 0 {
                let digit_mask = mask & (-mask);
                let digit = digit_mask.trailing_zeros() as i32;
                flip(i, j, digit, line, column, block);
                board[i][j] = ((digit + 1) as u8 + b'0') as char;
                dfs(pos + 1, spaces, line, column, block, board, valid);
                flip(i, j, digit, line, column, block);
                mask &= mask - 1;
                if *valid {
                    return;
                }
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    spaces.push(vec![i, j]);
                } else {
                    let digit = (board[i][j] as u8 - b'0') as i32 - 1;
                    line[i] ^= (1 << digit);
                    column[j] ^= (1 << digit);
                    block[i / 3][j / 3] ^= (1 << digit);
                }
            }
        }
        dfs(
            0,
            &spaces,
            &mut line,
            &mut column,
            &mut block,
            board,
            &mut valid,
        );
    }
}
// @lc code=end
