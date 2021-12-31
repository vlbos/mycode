/*
 * @lc app=leetcode id=909 lang=rust
 *
 * [909] Snakes and Ladders
 */

// @lc code=start
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let id2rc = |id: usize, n: usize| -> (usize, usize) {
            let r = (id - 1) / n;
            let mut c = (id - 1) % n;
            if r % 2 == 1 {
                c = n - 1 - c;
            }
            (n-1-r, c)
        };
        let n = board.len();
        let mut vis = vec![false;n*n+1];
        let mut q = std::collections::VecDeque::new();
        q.push_back((1, 0));
        while let Some(p) = q.pop_front() {
            for i in 1..=6 {
                let mut nxt = p.0 + i;
                if nxt > n * n {
                    break;
                }
                let (r,c) = id2rc(nxt,n);
                if board[r][c]>0{
                     nxt = board[r][c] as usize;
                }
                if nxt==n*n{
                    return p.1+1;
                }
                if !vis[nxt]{
                    vis[nxt]=true;
                    q.push_back((nxt,p.1+1));
                }
            }
        }
        -1
    }
}
// @lc code=end
