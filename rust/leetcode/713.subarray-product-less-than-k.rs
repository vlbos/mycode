/*
 * @lc app=leetcode id=713 lang=rust
 *
 * [713] Subarray Product Less Than K
 */

// @lc code=start
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut ans = 0;
        let mut l = 0;
        let mut p = 1;
        for r in 0..nums.len(){
            p*=nums[r];
            while p >= k  {
                    p/=nums[l];
                    l += 1;
            }
            ans+=r as i32-l as i32+1;
        }
        ans
    }
}
// @lc code=end
