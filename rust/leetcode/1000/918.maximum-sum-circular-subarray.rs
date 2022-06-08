/*
 * @lc app=leetcode id=918 lang=rust
 *
 * [918] Maximum Sum Circular Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        if nums.len()==1{
                return nums[0];
            }
             let sum =   nums.iter().sum::<i32>();
            let mut ans = i32::MIN;
            let mut cur = i32::MIN;
            for &v in &nums{
                cur = v+cur.max(0);
                ans = ans.max(cur);
            }
            let n = nums.len();
            let mut ans2 = i32::MAX;
            let mut cur2 = i32::MAX;
            for i in 1..n{
                  cur2=nums[i]+cur2.min(0);
                  ans2 = ans2.min(cur2);
            }
            ans2 = sum-ans2;

            let mut ans3 = i32::MAX;
            let mut cur3 = i32::MAX;
            for i in 0..n-1{
                  cur3=nums[i]+cur3.min(0);
                  ans3 = ans3.min(cur3);
            }
            // ans3 = sum-ans3;
            ans.max(ans2).max(ans3)
    }
}
// @lc code=end

