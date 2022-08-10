// 348\. Design Tic-Tac-Toe
// ========================

// Design a Tic-tac-toe game that is played between two players on a _n_ x _n_ grid.

// You may assume the following rules:

// 1.  A move is guaranteed to be valid and is placed on an empty block.
// 2.  Once a winning condition is reached, no more moves is allowed.
// 3.  A player who succeeds in placing _n_ of their marks in a horizontal, vertical, or diagonal row wins the game.

// **Example:**

// Given _n_ = 3, assume that player 1 is "X" and player 2 is "O" in the board.

// TicTacToe toe = new TicTacToe(3);

// toe.move(0, 0, 1); -> Returns 0 (no one wins)
// |X| | |
// | | | |    // Player 1 makes a move at (0, 0).
// | | | |

// toe.move(0, 2, 2); -> Returns 0 (no one wins)
// |X| |O|
// | | | |    // Player 2 makes a move at (0, 2).
// | | | |

// toe.move(2, 2, 1); -> Returns 0 (no one wins)
// |X| |O|
// | | | |    // Player 1 makes a move at (2, 2).
// | | |X|

// toe.move(1, 1, 2); -> Returns 0 (no one wins)
// |X| |O|
// | |O| |    // Player 2 makes a move at (1, 1).
// | | |X|

// toe.move(2, 0, 1); -> Returns 0 (no one wins)
// |X| |O|
// | |O| |    // Player 1 makes a move at (2, 0).
// |X| |X|

// toe.move(1, 0, 2); -> Returns 0 (no one wins)
// |X| |O|
// |O|O| |    // Player 2 makes a move at (1, 0).
// |X| |X|

// toe.move(2, 1, 1); -> Returns 1 (player 1 wins)
// |X| |O|
// |O|O| |    // Player 1 makes a move at (2, 1).
// |X|X|X|

// **Follow up:**
// Could you do better than O(_n_2) per `move()` operation?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [DoorDash](https://leetcode.ca/tags/#DoorDash) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [TripleByte](https://leetcode.ca/tags/#TripleByte) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
#[derive(Debug)]
pub  struct TicTacToe {
    // rows: Vec<Option<i32>>,
    // cols: Vec<Option<i32>>,
    // crosses: (Option<i32>, Option<i32>),
    // size: usize,
    board: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        // TicTacToe {
        //     rows: vec![Some(0); n as usize],
        //     cols: vec![Some(0); n as usize],
        //     crosses: (Some(0), Some(0)),
        //     size: n as usize,
        // }
        let n = n as usize;
        Self {
            board: vec![vec![0; n]; n],
        }
    }

    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        // let size = self.size;
        // if (TicTacToe::update_line(&mut self.rows[row as usize], player, size) == 0)
        //     && (TicTacToe::update_line(&mut self.cols[col as usize], player, size) == 0)
        //     && !(row == col && TicTacToe::update_line(&mut self.crosses.0, player, size) != 0)
        //     && !(row == (size as i32 - 1 - col)
        //         && TicTacToe::update_line(&mut self.crosses.1, player, size) != 0)
        // {
        //     0
        // } else {
        //     player
        // }
        let n = self.board.len();
        self.board[row as usize][col as usize] = player;
        //horizontal
        if self.board.iter().any(|x| x.iter().all(|v| *v == player)) {
            return player;
        }
        //vertical
        for j in 0..n {
            let mut win = true;
            for i in 0..n {
                if self.board[i][j] != player {
                    win = false;
                    break;
                }
            }
            if win {
                return player;
            }
        }
        //cross
        let (mut backslash, mut slash) = (0, 0);
        for i in 0..n {
            if i < backslash && i < slash {
                break;
            }
            if self.board[i][i] == player {
                backslash += 1;
            }
            if self.board[i][n - i - 1] == player {
                slash += 1;
            }
        }
        if backslash == n || slash == n {
            return player;
        }
        0
    }

    // fn update_line(line: &mut Option<i32>, player: i32, size: usize) -> i32 {
    //     let mut line_winner = 0;
    //     let next_line = match *line {
    //         Some(count) => {
    //             if player == 1 && count >= 0 {
    //                 let next_count = count + 1;
    //                 if next_count as usize == size {
    //                     line_winner = 1;
    //                 }
    //                 Some(next_count)
    //             } else if player == 2 && count <= 0 {
    //                 let next_count = count - 1;
    //                 if next_count == 0 - size as i32 {
    //                     line_winner = 2;
    //                 }
    //                 Some(next_count)
    //             } else {
    //                 None
    //             }
    //         }
    //         None => None,
    //     };
    //     *line = next_line;
    //     line_winner
    // }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tic_tac_toe() {
        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(0, 0, 1), 0);
        assert_eq!(toe.make_a_move(0, 2, 2), 0);
        assert_eq!(toe.make_a_move(2, 2, 1), 0);
        assert_eq!(toe.make_a_move(1, 1, 2), 0);
        assert_eq!(toe.make_a_move(2, 0, 1), 0);
        assert_eq!(toe.make_a_move(1, 0, 2), 0);
        assert_eq!(toe.make_a_move(2, 1, 1), 1);
    }
}
