/*
 * @lc app=leetcode id=154 lang=rust
 *
 * [154] Find Minimum in Rotated Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut h) = (0, nums.len() - 1);
        while l < h {
            let pivot = (l + h) / 2;
            if nums[pivot] < nums[h] {
                h = pivot;
            } else if nums[pivot] > nums[h] {
                l = pivot + 1;
            } else {
                h -= 1;
            }
        }
        nums[l]
    }
}
// @lc code=end
