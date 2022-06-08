/*
 * @lc app=leetcode id=1968 lang=rust
 *
 * [1968] Array With Elements Not Equal to Average of Neighbors
 */

// @lc code=start
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let m = (n + 1) / 2;
        let mut ans = Vec::new();
        for i in 0..m {
            ans.push(nums[i]);
            if i + m < n {
                ans.push(nums[i + m]);
            }
        }
        ans
    }
}
// @lc code=end
