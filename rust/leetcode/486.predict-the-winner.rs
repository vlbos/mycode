/*
 * @lc app=leetcode id=486 lang=rust
 *
 * [486] Predict the Winner
 */

// @lc code=start
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        // fn sum(nums: &Vec<i32>,i:usize,j:usize,k:i32)->i32{
        //     if i==j{
        //     return  nums[i]*k;
        //     }
        //     let ii = nums[i]*k+sum(nums,i+1,j,-k);
        //     let jj = nums[j]*k+sum(nums,i,j-1,-k);
        //     (ii*k).max(jj*k)*k
        // }
        // sum(&nums,0,nums.len()-1,1)>=0
        let mut dp = nums.clone();
        for i in (0..nums.len()-1).rev(){
            for j in i+1..nums.len(){
                dp[j]= (nums[i]-dp[j]).max(nums[j]-dp[j-1]);
            }
        }
        *dp.last().unwrap()>=0
    }
}
// @lc code=end
