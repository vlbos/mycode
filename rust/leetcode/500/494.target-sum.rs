/*
 * @lc app=leetcode id=494 lang=rust
 *
 * [494] Target Sum
 */

// @lc code=start
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &Vec<i32>,idx:usize, sum:i32,target: i32,ans:&mut i32){
            if idx==nums.len(){
                if sum==target{
                *ans+=1;
                }
                return;
            }
            dfs(nums,idx+1,sum+nums[idx],target,ans);
            dfs(nums,idx+1,sum-nums[idx],target,ans);
        }
        let mut ans=0;
         dfs(&nums,0,0,target,&mut ans);
        ans
    }
}
// @lc code=end

