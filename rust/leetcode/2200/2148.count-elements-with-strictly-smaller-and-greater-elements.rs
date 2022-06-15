/*
 * @lc app=leetcode id=2148 lang=rust
 *
 * [2148] Count Elements With Strictly Smaller and Greater Elements
 */

// @lc code=start
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
         let  max = *nums.iter().max().unwrap();
        let  min = *nums.iter().min().unwrap();
        nums.into_iter()
            .filter(|v| *v != min && *v != max)
            .count() as _
    }
}
// @lc code=end
