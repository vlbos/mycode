/*
 * @lc app=leetcode id=152 lang=rust
 *
 * [152] Maximum Product Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut minf = nums[0];
        let mut maxf = nums[0];
        for i in 1..nums.len(){
            let mx = maxf;
            let mn = minf;
            maxf = nums[i].max(mn*nums[i]).max(mx*nums[i]);
            minf = nums[i].min(mx*nums[i]).min(mn*nums[i]);
            ans=ans.max(maxf);
        }
        ans
    }
}
// @lc code=end

