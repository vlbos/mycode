/*
 * @lc app=leetcode id=2089 lang=rust
 *
 * [2089] Find Target Indices After Sorting Array
 */

// @lc code=start
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut ans = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if target == num {
                ans.push(i as i32);
            }
        }
        ans
    }
}
// @lc code=end
