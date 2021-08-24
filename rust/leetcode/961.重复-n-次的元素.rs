/*
 * @lc app=leetcode.cn id=961 lang=rust
 *
 * [961] 重复 N 次的元素
 */

// @lc code=start
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 1..=3{
           for j in 0..nums.len()-i{
            if nums[j]==nums[j+i]{
                        return nums[j]
                        }
           }
        }
        0
    }
}
// @lc code=end

