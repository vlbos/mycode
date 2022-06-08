/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ans = 0;
        let mut end = 0;
        for i in 0..nums.len()-1{
            max = max.max(i+nums[i] as usize);
            if i==end{
                end=max;
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

