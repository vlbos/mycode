/*
 * @lc app=leetcode id=1838 lang=rust
 *
 * [1838] Frequency of the Most Frequent Element
 */

// @lc code=start
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut l = 0;
        let mut ans = 1;
        let mut total = 0;
        for r in 1..nums.len() {
            total += (nums[r] - nums[r - 1]) as i64 * (r - l) as i64;
            while total > k as i64 {
                total -= (nums[r] - nums[l]) as i64;
                l += 1;
            }
            ans = ans.max((r - l + 1) as i32);
        }
        ans
    }
}
// @lc code=end
