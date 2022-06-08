/*
 * @lc app=leetcode id=96 lang=rust
 *
 * [96] Unique Binary Search Trees
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
         let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}
// @lc code=end

