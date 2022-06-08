/*
 * @lc app=leetcode id=1928 lang=rust
 *
 * [1928] Minimum Cost to Reach Destination in Time
 */

// @lc code=start
impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let inf = i32::MAX / 2;
        let mt = max_time as usize;
        let mut f = vec![vec![inf; n]; mt + 1];
        f[0][0] = passing_fees[0];
        for t in 1..=mt {
            for edge in &edges {
                let (i, j, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);
                if cost <= t {
                    f[t][i] = f[t][i].min(f[t - cost][j] + passing_fees[i]);
                    f[t][j] = f[t][j].min(f[t - cost][i] + passing_fees[j]);
                }
            }
        }

        let ans = f.iter().min_by_key(|x| x[n - 1]).unwrap()[n - 1];
        if ans == inf {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
