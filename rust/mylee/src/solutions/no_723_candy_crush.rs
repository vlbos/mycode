// 723\. Candy Crush
// =================

// This question is about implementing a basic elimination algorithm for Candy Crush.

// Given a 2D integer array `board` representing the grid of candy, different positive integers `board[i][j]` represent different types of candies.
// A value of `board[i][j] = 0` represents that the cell at position `(i, j)` is empty.
// The given board represents the state of the game following the player's move.
// Now, you need to restore the board to a _stable state_ by crushing candies according to the following rules:

// 1.  If three or more candies of the same type are adjacent vertically or horizontally, "crush" them all at the same time - these positions become empty.
// 2.  After crushing all candies simultaneously, if an empty space on the board has candies on top of itself,
//  then these candies will drop until they hit a candy or bottom at the same time. (No new candies will drop outside the top boundary.)
// 3.  After the above steps, there may exist more candies that can be crushed. If so, you need to repeat the above steps.
// 4.  If there does not exist more candies that can be crushed (ie. the board is _stable_), then return the current board.

// You need to perform the above rules until the board becomes stable, then return the current board.

// **Example:**

// **Input:**
// board =
// \[\[110,5,112,113,114\],\[210,211,5,213,214\],\[310,311,3,313,314\],\[410,411,412,5,414\],\[5,1,512,3,3\],\[610,4,1,613,614\],\[710,1,2,713,714\],\[810,1,2,1,1\],\[1,1,2,2,2\],\[4,1,4,4,1014\]\]

// **Output:**
// \[\[0,0,0,0,0\],\[0,0,0,0,0\],\[0,0,0,0,0\],\[110,0,0,0,114\],\[210,0,0,0,214\],\[310,0,0,113,314\],\[410,0,0,213,414\],\[610,211,112,313,614\],\[710,311,412,613,714\],\[810,411,512,713,1014\]\]

// **Explanation:**
// ![](https://leetcode.ca/all/img/723.png)

// **Note:**

// 1.  The length of `board` will be in the range \[3, 50\].
// 2.  The length of `board[i]` will be in the range \[3, 50\].
// 3.  Each `board[i][j]` will initially start as an integer in the range \[1, 2000\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Google](https://leetcode.ca/tags/#Google) [Rubrik](https://leetcode.ca/tags/#Rubrik)

// @lc code=start
impl Solution {
    pub fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // let rows = board.len();
        // let cols = if rows == 0 { 0 } else { board[0].len() };
        // if rows * cols == 0 {
        //     return board;
        // }
        // loop {
        //     let mut crush_size = 0;
        //     // crush by row
        //     for i in 0..rows {
        //         let mut same_count = (0, 0);
        //         for j in 0..=cols {
        //             let current = if j == cols {
        //                 i32::max_value()
        //             } else {
        //                 board[i][j]
        //             };
        //             let (last, count) = same_count;
        //             if last == i32::abs(current) && current != 0 {
        //                 same_count.1 += 1;
        //             } else {
        //                 if count >= 3 && last != 0 {
        //                     for k in (j - count)..j {
        //                         let b = board[i][k];
        //                         board[i][k] = if b > 0 {
        //                             crush_size += 1;
        //                             0 - b
        //                         } else {
        //                             b
        //                         }
        //                     }
        //                 }
        //                 same_count.0 = i32::abs(current);
        //                 same_count.1 = 1;
        //             }
        //         }
        //     }
        //     // crush by col
        //     for j in 0..cols {
        //         let mut same_count = (0, 0);
        //         for i in 0..=rows {
        //             let current = if i == rows {
        //                 i32::max_value()
        //             } else {
        //                 board[i][j]
        //             };
        //             let (last, count) = same_count;
        //             if last == i32::abs(current) && current != 0 {
        //                 same_count.1 += 1;
        //             } else {
        //                 if count >= 3 && last != 0 {
        //                     for k in (i - count)..i {
        //                         let b = board[k][j];
        //                         board[k][j] = if b > 0 {
        //                             crush_size += 1;
        //                             0 - b
        //                         } else {
        //                             b
        //                         }
        //                     }
        //                 }
        //                 same_count.0 = i32::abs(current);
        //                 same_count.1 = 1;
        //             }
        //         }
        //     }
        //     // drop by col
        //     for j in 0..cols {
        //         let mut k = (rows - 1) as i32;
        //         for i in (0..rows).rev() {
        //             let b = board[i][j];
        //             if b > 0 {
        //                 board[k as usize][j] = board[i][j];
        //                 k -= 1;
        //             }
        //         }
        //         for i in (0..=k).rev() {
        //             board[i as usize][j] = 0;
        //         }
        //     }
        //     if crush_size == 0 {
        //         break;
        //     }
        // }
        // return board;
        let (m, n) = (board.len(), board[0].len());
        let mut crush = false;

        for i in 0..m {
            for j in 0..n {
                let v = board[i][j].abs();
                if v == 0 {
                    continue;
                }
                if j + 2 < n && v == board[i][j + 1].abs() && v == board[i][j + 2].abs() {
                    crush = true;
                    for k in 0..3 {
                        board[i][j + k] = -v;
                    }
                }
                if i + 2 < m && v == board[i + 1][j].abs() && v == board[i + 2][j].abs() {
                    crush = true;
                    for k in 0..3 {
                        board[i + k][j] = -v;
                    }
                }
            }
        }
        for j in 0..n {
            let mut k = m;
            for i in (0..m).rev() {
                if board[i][j] > 0 {
                    board[k - 1][j] = board[i][j];
                    k -= 1;
                }
            }
            while k > 0 {
                board[k - 1][j] = 0;
                k -= 1;
            }
        }
        if crush {
            Self::candy_crush(board)
        } else {
            board
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_candy_crush_1() {
        let board = lc_matrix![
            [110, 5, 112, 113, 114],
            [210, 211, 5, 213, 214],
            [310, 311, 3, 313, 314],
            [410, 411, 412, 5, 414],
            [5, 1, 512, 3, 3],
            [610, 4, 1, 613, 614],
            [710, 1, 2, 713, 714],
            [810, 1, 2, 1, 1],
            [1, 1, 2, 2, 2],
            [4, 1, 4, 4, 1014]
        ];
        let expect = lc_matrix![
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [110, 0, 0, 0, 114],
            [210, 0, 0, 0, 214],
            [310, 0, 0, 113, 314],
            [410, 0, 0, 213, 414],
            [610, 211, 112, 313, 614],
            [710, 311, 412, 613, 714],
            [810, 411, 512, 713, 1014]
        ];
        let res = Solution::candy_crush(board);
        assert_eq!(expect, res);
    }
}
