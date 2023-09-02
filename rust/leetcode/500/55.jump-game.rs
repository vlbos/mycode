/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len()<=1{
            return true;
        } 
        let mut max = nums[0] as usize;
        for (i, n) in nums.iter().enumerate() {
            if i>max{
                return false;
            }
            max = max.max(i + *n as usize);
            if max >= nums.len() - 1 {
                return true;
            }
        }
        false
    }
}
// @lc code=end
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n=nums.len();
        if n==1{
            return true
        }
        if nums[0]==0{
            return false
        }
        let mut dp=vec![0;n];
        dp[0]=nums[0] as usize;
        for i in 1..n-1{
            dp[i]=dp[i-1].max(nums[i] as usize+i);
            if dp[i]>=n-1{
                return true
            }
            if dp[i]==i{
                return false
            }
        }
        true
    }
}