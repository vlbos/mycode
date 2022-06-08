/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 */

// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut vis = vec![false; n * m];
        let mut ans = false;
        let mut w = word.chars().collect::<Vec<char>>();
        let mut res = Vec::new();
        fn back_track(
            board: &Vec<Vec<char>>,
            w: &Vec<char>,
            mut vis: &mut Vec<bool>,
            mut res: &mut Vec<char>,
            mut flag: &mut bool,
            idx: usize,
        ) {
            if res == w {
                *flag = true;
                return;
            }
            if *flag || res.len() == w.len() || idx == board.len() * board[0].len() {
                return;
            }
            let mut x = idx / board[0].len();
            let mut y = idx % board[0].len();
            if x < board.len() - 1 {
                let x1 = x + 1;
                let idx = x1 * board[0].len() + y;
                if !vis[idx] {
                    vis[idx] = true;
                    res.push(board[x1][y]);
                    back_track(board, w, vis, res, flag, idx);
                    vis[idx] = false;
                    res.pop();
                }
            }
            if y < board[0].len() - 1 {
                let y1 = y + 1;
                let idx = x * board[0].len() + y1;
                if !vis[idx] {
                    vis[idx] = true;
                    res.push(board[x][y1]);
                    back_track(board, w, vis, res, flag, idx);
                    vis[idx] = false;
                    res.pop();
                }
            }
            if x > 0 {
                let x1 = x - 1;
                let idx = x1 * board[0].len() + y;
                if !vis[idx] {
                    vis[idx] = true;
                    res.push(board[x1][y]);
                    back_track(board, w, vis, res, flag, idx);
                    vis[idx] = false;
                    res.pop();
                }
            }
            if y > 0 {
                let y1 = y - 1;
                let idx = x * board[0].len() + y1;
                if !vis[idx] {
                    vis[idx] = true;
                    res.push(board[x][y1]);
                    back_track(board, w, vis, res, flag, idx);
                    vis[idx] = false;
                    res.pop();
                }
            }
        }
        for i in 0..board.len() * board[0].len() {
            if vis[i] {
                continue;
            }
            vis[i] = true;
            res.push(board[i / board[0].len()][i % board[0].len()]);
            back_track(&board, &w, &mut vis, &mut res, &mut ans, i);
            if ans {
                return true;
            }
            vis[i] = false;
            res.pop();
        }
        ans
    }
}
// @lc code=end
