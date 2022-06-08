/*
 * @lc app=leetcode id=741 lang=rust
 *
 * [741] Cherry Pickup
 */

// @lc code=start
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![i32::MIN; n]; n];
        dp[0][0] = grid[0][0];
        for t in 1..=2 * (n - 1) {
            let mut dp2 = vec![vec![i32::MIN; n]; n];
            let tt = if t > n - 1 { t - (n - 1) } else { 0 };
            for i in tt..=t.min(n - 1) {
                for j in tt..=t.min(n - 1) {
                    if grid[i][t - i] == -1 || grid[j][t - j] == -1 {
                        continue;
                    }
                    let mut ans = grid[i][t - i];
                    if i != j {
                        ans += grid[j][t - j];
                    }
                    let (mut ii, mut jj)=(i,j);
                    if ii>0{
                        ii-=1;
                    }
                    if jj>0{
                        jj-=1;
                    }
                    for pi in ii..=i {
                        for pj in jj..=j {
                            dp2[i][j] = dp2[i][j].max(dp[pi][pj] + ans);
                        }
                    }
                }
            }
            dp = dp2;
        }
        dp[n - 1][n - 1].max(0)
    }
}
// @lc code=end
