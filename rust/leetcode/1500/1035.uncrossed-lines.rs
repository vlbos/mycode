/*
 * @lc app=leetcode id=1035 lang=rust
 *
 * [1035] Uncrossed Lines
 */

// @lc code=start
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (n1,n2)=(nums1.len(),nums2.len());
        let mut dp=vec![vec![0;n2+1];n1+1];
        for i in 1..=n1{
            for j in 1..=n2{
                if nums1[i-1]==nums2[j-1]{
                    dp[i][j]=dp[i-1][j-1]+1;

                }else{
                    dp[i][j]=dp[i][j-1].max(dp[i-1][j]);
                }
            }
        }
        dp[n1][n2]
    }
}
// @lc code=end

