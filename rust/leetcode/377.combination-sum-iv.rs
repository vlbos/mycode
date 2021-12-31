/*
 * @lc app=leetcode id=377 lang=rust
 *
 * [377] Combination Sum IV
 */

// @lc code=start
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0;target as usize+1];
        dp[0]=1;
        for i in 1..=target{
            for n in &nums{
                if *n<=i{
                    dp[i as usize]+=dp[(i-*n) as usize];
                }
            }
        }
        dp[target as usize]
    }
}
// @lc code=end

