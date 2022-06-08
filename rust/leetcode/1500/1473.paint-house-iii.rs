/*
 * @lc app=leetcode id=1473 lang=rust
 *
 * [1473] Paint House III
 */

// @lc code=start
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
         let houses: Vec<i32> = houses.into_iter().map(|c| c - 1).collect();
        let (m, n, t) = (m as usize, n as usize, target as usize);
        let inf = i32::MAX / 2;
        let mut dp = vec![vec![vec![inf; t]; n]; m];
        let mut best = vec![vec![vec![inf, -1, inf]; t]; m];
        for i in 0..m {
            for j in 0..n {
                if houses[i] != -1 && houses[i] != j as i32 {
                    continue;
                }
                for k in 0..t {
                    if i == 0 {
                        if k == 0 {
                            dp[i][j][k] = 0;
                        }
                    } else {
                        dp[i][j][k] = dp[i - 1][j][k];
                        if k > 0 {
                            let ik = &best[i - 1][k - 1];
                            dp[i][j][k] =
                                dp[i][j][k].min(if ik[1] == j as i32 { ik[2] } else { ik[0] });
                        }
                    }
                    if dp[i][j][k] != inf && houses[i] == -1 {
                        dp[i][j][k] += cost[i][j];
                    }
                    if dp[i][j][k] < best[i][k][0] {
                        best[i][k][2] = best[i][k][0];
                        best[i][k][0] = dp[i][j][k];
                        best[i][k][1] = j as i32;
                    } else if  dp[i][j][k] < best[i][k][2]{
                        best[i][k][2] = dp[i][j][k];
                    }
                }
            }
        }
        let ans = dp[m - 1].iter().min_by_key(|x| x[t - 1]).unwrap()[t - 1];
        if ans == inf {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
