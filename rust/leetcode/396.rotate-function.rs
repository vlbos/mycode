/*
 * @lc app=leetcode id=396 lang=rust
 *
 * [396] Rotate Function
 */

// @lc code=start
impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut sum = 0;
        let mut fx=0;
        let n = nums.len();
        for (i,v) in nums.iter().enumerate(){
            fx+=i as i32 *v;
            sum+=v;
        }
        max = fx;
        for i in 1..nums.len(){
            fx=fx+sum-nums[n-i]*n as i32;
            max=max.max(fx);
        }
        max
    }
}
// @lc code=end

