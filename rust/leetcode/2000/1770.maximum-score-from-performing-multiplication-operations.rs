/*
 * @lc app=leetcode id=1770 lang=rust
 *
 * [1770] Maximum Score from Performing Multiplication Operations
 */

// @lc code=start
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut memo = vec![vec![std::i32::MIN; m]; n];
        fn dp(
            k: usize,
            i: usize,
            nums: &Vec<i32>,
            muls: &Vec<i32>,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == muls.len() {
                return 0;
            }
            if memo[k][i] != std::i32::MIN {
                return memo[k][i];
            }
            let left = dp(k + 1, i + 1, nums, muls, memo) + nums[k] * muls[i];
            let right = dp(k, i + 1, nums, muls, memo) + nums[nums.len() - i + k - 1] * muls[i];
            memo[k][i] = left.max(right);
            memo[k][i]
        }
        dp(0, 0, &nums, &multipliers, &mut memo)
    }
}
// @lc code=end
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (m,n)=(multipliers.len(),nums.len());
        let mut dp=vec![vec![0;n+1];m+1];
        let mut ans=i32::MIN;
        for j in 1..=m{
            for i in 0..=j{
                if i==0{
                    dp[i][j-i]=dp[i][j-i-1]+nums[n+i-j]*multipliers[j-1];
                }else if i==j{
                    dp[i][j-i]=dp[i-1][j-i]+nums[i-1]*multipliers[j-1];
                }else{
                    dp[i][j-i]=(dp[i][j-i-1]+nums[n+i-j]*multipliers[j-1]).max(dp[i-1][j-i]+nums[i-1]*multipliers[j-1]);
                }
                if j==m{
                    ans=ans.max(dp[i][j-i]);
                }
            }
        }
        ans
    }
}