/*
 * @lc app=leetcode id=376 lang=rust
 *
 * [376] Wiggle Subsequence
 */

// @lc code=start
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut pd = nums[1] - nums[0];
        let mut ans = if pd != 0 { 2 } else { 1 };
        for i in 2..nums.len() {
            let d = nums[i] - nums[i - 1];
            if d < 0 && pd >= 0 || d > 0 && pd <= 0 {
                ans += 1;
                pd = d;
            }
        }
        ans
    }
}
// @lc code=end
