/*
 * @lc app=leetcode id=1301 lang=rust
 *
 * [1301] Number of Paths with Max Score
 */

// @lc code=start
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let mut dp = vec![vec![vec![-1, 0]; n]; n];
        dp[n - 1][n - 1] = vec![0, 1];
        let p = 1_000_000_007;
        let update = |dp: &mut Vec<Vec<Vec<i32>>>, x: usize, y: usize, u: usize, v: usize| {
            if u >= n || v >= n || dp[u][v][0] == -1 {
                return;
            }
            if dp[u][v][0] > dp[x][y][0] {
                dp[x][y] = dp[u][v].clone();
            } else if dp[u][v][0] == dp[x][y][0] {
                dp[x][y][1] += dp[u][v][1];
                if dp[x][y][1] >= p {
                    dp[x][y][1] -= p;
                }
            }
        };
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if i == n - 1 && j == n - 1 || board[i].as_bytes()[j] == b'X' {
                    continue;
                }
                update(&mut dp, i, j, i + 1, j);
                update(&mut dp, i, j, i, j + 1);
                update(&mut dp, i, j, i + 1, j + 1);
                if dp[i][j][0] != -1 {
                    dp[i][j][0] += if board[i].as_bytes()[j] == b'E' {
                        0
                    } else {
                        (board[i].as_bytes()[j] - b'0') as i32
                    };
                }
            }
        }
        if dp[0][0][0] == -1 {
            vec![0, 0]
        } else {
            dp[0][0].clone()
        }
    }
}
// @lc code=end
