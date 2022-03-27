/*
 * @lc app=leetcode id=1958 lang=rust
 *
 * [1958] Check if Move is Legal
 */

// @lc code=start
impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let check = |dx: i32, dy: i32| -> bool {
            let (mut x, mut y) = (r_move + dx, c_move + dy);
            let mut flag = true;
            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let (xx, yy) = (x as usize, y as usize);
                if flag {
                    if board[xx][yy] == '.' || board[xx][yy] == color {
                        return false;
                    }
                    flag = false;
                } else {
                    if board[xx][yy] == '.' {
                        return false;
                    }
                    if board[xx][yy] == color {
                        return true;
                    }
                }
                x += dx;
                y += dy;
            }
            false
        };
        for i in (-1..=1) {
            for j in (-1..=1) {
                if i == 0 && j == 0 {
                    continue;
                }
                if check(i, j) {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
