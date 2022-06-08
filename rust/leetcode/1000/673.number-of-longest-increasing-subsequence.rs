/*
 * @lc app=leetcode id=673 lang=rust
 *
 * [673] Number of Longest Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut maxlen = 0;
        let mut dp = vec![0; nums.len()];
        let mut cnt = vec![0; nums.len()];
        for i in 0..nums.len() {
            dp[i] = 1;
            cnt[i] = 1;
            for j in 0..i {
                if nums[j] < nums[i] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        cnt[i] = cnt[j];
                    } else if dp[j] + 1 == dp[i] {
                        cnt[i] += cnt[j];
                    }
                }
            }
            if maxlen < dp[i] {
                maxlen = dp[i];
                ans = cnt[i];
            } else if maxlen == dp[i] {
                ans += cnt[i];
            }
        }
        ans
    }
}
// @lc code=end
