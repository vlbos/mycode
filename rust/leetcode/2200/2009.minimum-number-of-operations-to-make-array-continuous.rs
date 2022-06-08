/*
 * @lc app=leetcode id=2009 lang=rust
 *
 * [2009] Minimum Number of Operations to Make Array Continuous
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut nums = nums;
        nums.sort();
        nums.dedup();
        let mut ans = 0;
        for (r, v) in nums.iter().enumerate() {
            if let Ok(l) | Err(l) = nums[..r].binary_search(&(v - n + 1)) {
                ans = ans.max((r + 1 - l) as i32);
            }
        }
        n - ans
    }
}
// @lc code=end
