/*
 * @lc app=leetcode id=529 lang=rust
 *
 * [529] Minesweeper
 */

// @lc code=start
impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let x = click[0] as usize;
        let y = click[1] as usize;
        if board[x][y] == 'M' {
            board[x][y] = 'X';
            return board;
        }
        let ds = vec![-1, 0, 1];
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let mut q = std::collections::VecDeque::new();
        q.push_back(vec![x as i32, y as i32]);
        let mut vis = vec![vec![false; n as usize]; m as usize];
        vis[x][y] = true;
        while let Some(qq) = q.pop_front() {
            let r = qq[0];
            let c = qq[1];
            let mut bb = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    let rr = r + ds[i];
                    let cc = c + ds[j];
                    if rr >= 0 && rr < m && cc >= 0 && cc < n {
                        let rrr = rr as usize;
                        let ccc = cc as usize;
                        if board[rrr][ccc] == 'X' || board[rrr][ccc] == 'M' {
                            bb += 1;
                        }
                    }
                }
            }
            let ru = r as usize;
            let cu = c as usize;
            if bb > 0 {
                board[ru][cu] = (b'0' + bb as u8) as char;
            } else {
                board[ru][cu] = 'B';
                for i in 0..3 {
                    for j in 0..3 {
                        if i == 1 && j == 1 {
                            continue;
                        }
                        let rr = r + ds[i];
                        let cc = c + ds[j];
                        if rr >= 0 && rr < m && cc >= 0 && cc < n {
                            let rrr = rr as usize;
                            let ccc = cc as usize;
                            if !vis[rrr][ccc] && board[rrr][ccc] == 'E' {
                                q.push_back(vec![rr, cc]);
                                vis[rrr][ccc] = true;
                            }
                        }
                    }
                }
            }
        }

        board
    }
}
// @lc code=end
