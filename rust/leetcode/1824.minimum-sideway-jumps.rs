/*
 * @lc app=leetcode id=1824 lang=rust
 *
 * [1824] Minimum Sideway Jumps
 */

// @lc code=start
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp = vec![1; 3];
        dp[1] = 0;
        for &o in obstacles.iter().skip(1) {
            let pre = dp.clone();
            dp.fill(i32::MAX / 2);
            for i in 0..3 {
                if o as usize != i + 1 {
                    dp[i] = pre[i];
                }
            }
            for i in 0..3 {
                if o as usize != i + 1 {
                    let (k, j) = ((i + 1) % 3, (i + 2) % 3);
                    dp[i] = dp[i].min(dp[j].min(dp[k]) + 1);
                }
            }
        }

        *dp.iter().min().unwrap()
    }
}
// @lc code=end
