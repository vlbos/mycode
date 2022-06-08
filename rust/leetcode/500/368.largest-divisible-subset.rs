/*
 * @lc app=leetcode id=368 lang=rust
 *
 * [368] Largest Divisible Subset
 */

// @lc code=start
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len()==1{
        return nums;
        }
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut dp = vec![1; n];
        let mut maxsize = 0;
        let mut maxval = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
                if dp[i] > maxsize {
                    maxsize = dp[i];
                    maxval = nums[i];
                }
            }
        }
        let mut ans = Vec::new();
        for i in (0..n).rev() {
            if dp[i] == maxsize && maxval % nums[i] == 0 {
                ans.push(nums[i]);
                maxsize -= 1;
                maxval=nums[i];
            }
           
            if maxsize == 0 {
                break;
            }
        }
        ans
    }
}
// @lc code=end
