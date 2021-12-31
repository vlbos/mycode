/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        if nums.is_empty() {
            return ans;
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r && nums[l] != target {
            if l == nums.len() - 1 {
                break;
            }
            l += 1;
        }
        if nums[l] == target {
            ans[0] = l as i32;
        }
        while l <= r && nums[r] != target {
            if r == 0 {
                break;
            }
            r -= 1;
        }
        if nums[r] == target {
            ans[1] = r as i32;
        }
        ans
    }
}
// @lc code=end
