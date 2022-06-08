/*
 * @lc app=leetcode id=1004 lang=rust
 *
 * [1004] Max Consecutive Ones III
 */

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let  (mut lsum,mut rsum)=(0,0);
        for (r,v) in nums.iter().enumerate(){
             rsum+=1-v;
            while lsum<rsum-k{
                lsum+=1-nums[l];
                l+=1;
            }
            ans = ans.max(r-l+1);
        }
        ans as _
    }
}
// @lc code=end

