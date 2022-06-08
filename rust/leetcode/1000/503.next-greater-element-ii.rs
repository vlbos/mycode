/*
 * @lc app=leetcode id=503 lang=rust
 *
 * [503] Next Greater Element II
 */

// @lc code=start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::new();
        let n = nums.len();
        let mut ans =vec![-1;n];
        for i in 0..nums.len()*2-1{
            let ii = i%n;
             while !s.is_empty() &&  nums[*s.last().unwrap()]<nums[ii]{
                    ans[*s.last().unwrap()]=nums[ii];
                    s.pop();
                }
              s.push(ii);
        }
        ans
    }
}
// @lc code=end

