/*
 * @lc app=leetcode id=51 lang=rust
 *
 * [51] N-Queens
 */

// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut ans = Vec::new();
        let mut queens = vec![n; n];
        fn solve(
            row: usize,
            columns: i32,
            diagonals1: i32,
            diagonals2: i32,
            queens: &mut Vec<usize>,
            ans: &mut Vec<Vec<String>>,
        ) {
            let generate_board = |queens: &mut Vec<usize>| -> Vec<String> {
                let mut ans = Vec::new();
                let n = queens.len();
                for &mut i in queens {
                    let mut row = vec!['.'; n];
                    row[i] = 'Q';
                    ans.push(row.iter().collect());
                }
                ans
            };
            let n = queens.len();

            if row == n {
                let board = generate_board(queens);
                ans.push(board);
                return;
            }
            let mut ap = ((1 << n) - 1) & (!(columns | diagonals1 | diagonals2));
            while ap != 0 {
                let p = ap & (-ap);
                ap &= (ap - 1);
                queens[row] = p.trailing_zeros() as usize;
                solve(
                    row + 1,
                    columns | p,
                    (diagonals1 | p) >> 1,
                    (diagonals2 | p) << 1,
                    queens,
                    ans,
                );
                queens[row] = n;
            }
        }
        solve(0,0,0,0,&mut queens,&mut ans);
        ans
    }
}
// @lc code=end
