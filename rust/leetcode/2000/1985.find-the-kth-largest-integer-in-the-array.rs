/*
 * @lc app=leetcode id=1985 lang=rust
 *
 * [1985] Find the Kth Largest Integer in the Array
 */

// @lc code=start
impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums = nums;
        nums.sort_by(|a, b| {
            if a.len() == b.len() {
                b.cmp(&a)
            } else {
                b.len().cmp(&a.len())
            }
        });
        nums[k as usize - 1].clone()
    }
}
// @lc code=end
