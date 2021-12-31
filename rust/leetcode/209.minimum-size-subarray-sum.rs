/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut sum = nums.iter().sum::<i32>();
        if sum < target {
            return 0;
        }
        let mut i = 0;
        let mut j = 0;
        sum = 0;
        let mut ans = i32::MAX;
        while j < nums.len() {
            sum += nums[j];
            while sum >= target {
                ans = ans.min((j - i + 1) as i32);
                sum -= nums[i];
                i += 1;
            }
            j += 1;
        }
        ans
    }
}
// @lc code=end
