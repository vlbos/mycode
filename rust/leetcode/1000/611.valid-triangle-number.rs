/*
 * @lc app=leetcode id=611 lang=rust
 *
 * [611] Valid Triangle Number
 */

// @lc code=start
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        nums.sort();
        let mut ans = 0;
        for i in 0..n{
            let mut k =i;
            for j in i+1..n{
                while k+1<n && nums[k+1]<nums[i]+nums[j]{
                    k+=1;
                }
                ans += if k>j {k-j}else{0};
            }
        }
        ans as i32
    }
}
// @lc code=end

