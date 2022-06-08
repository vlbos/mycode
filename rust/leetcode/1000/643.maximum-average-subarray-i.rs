/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
         let mut max = 0f64;
        let k = k as usize;
        for i in 0..k {
            max += nums[i] as f64;
        }
        let mut s = max;
        for i in k..nums.len() {
            s += nums[i] as f64;
            s -= nums[i - k] as f64;
            if s > max {
                max = s;
            }
        }
        max / k as f64
    }
}
// @lc code=end

