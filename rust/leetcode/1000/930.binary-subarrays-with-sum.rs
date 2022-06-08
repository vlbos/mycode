/*
 * @lc app=leetcode id=930 lang=rust
 *
 * [930] Binary Subarrays With Sum
 */

// @lc code=start
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let (mut l1,mut l2 ,mut r )=(0,0,0);
        let (mut sum1,mut sum2 )=(0,0);
        let mut ans = 0;
        while r <n{
            sum1+=nums[r];
            while l1<=r && sum1>goal{
                sum1-=nums[l1];
                l1+=1;
            }
            sum2+=nums[r];
             while l2<=r && sum2>=goal{
                sum2-=nums[l2];
                l2+=1;
            }
            ans+=l2-l1;
            r+=1;
        }
        ans as _
    }
}
// @lc code=end

