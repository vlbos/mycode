/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1;nums.len()];
        let mut pre = 1;
        for (i,n) in nums.iter().enumerate(){
            ans[i]*=pre;
            pre*=n;
        }
        pre = 1;
        for (i,n) in nums.iter().rev().enumerate(){
            ans[nums.len()-i-1]*=pre;
            pre*=n;
        }
        ans
    }
}
// @lc code=end

