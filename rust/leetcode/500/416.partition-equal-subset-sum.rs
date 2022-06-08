/*
 * @lc app=leetcode id=416 lang=rust
 *
 * [416] Partition Equal Subset Sum
 */

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let sum = nums.iter().sum::<i32>();
        let max = *(nums.iter().max().unwrap());
        if sum & 1 == 1 {
            return false;
        }
        let target = sum / 2;
        if max > target {
            return false;
        }
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for i in 1..nums.len() {
            for j in (nums[i]..=target).rev() {
                let j = j as usize;
                dp[j] = dp[j] || dp[j - nums[i] as usize];
            }
        }
        dp[target as usize]
    }
}
// @lc code=end
