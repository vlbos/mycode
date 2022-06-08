/*
 * @lc app=leetcode id=1275 lang=rust
 *
 * [1275] Find Winner on a Tic Tac Toe Game
 */

// @lc code=start
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut r = vec![vec![0; 3]; 2];
        let mut c = vec![vec![0; 3]; 2];
        let mut d = vec![vec![0; 2]; 2];
        let mut ii = 0;
        for (i, m) in moves.iter().enumerate() {
            ii = i % 2;
            r[ii][m[0] as usize] += 1;
            c[ii][m[1] as usize] += 1;
            if m[0] == m[1] {
                d[ii][0] += 1;
            }
            if (m[0] == 0 && m[1] == 2) || (m[1] == 0 && m[0] == 2) || (m[1] == 1 && m[0] == 1) {
                d[ii][1] += 1;
            }
            if r[ii][m[0] as usize] == 3
                || c[ii][m[1] as usize] == 3
                || d[ii][0] == 3
                || d[ii][1] == 3
            {
                return if ii == 0 {
                    "A".to_string()
                } else {
                    "B".to_string()
                };
            }
        }
        if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}
// @lc code=end
