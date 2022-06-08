/*
 * @lc app=leetcode id=689 lang=rust
 *
 * [689] Maximum Sum of 3 Non-Overlapping Subarrays
 */

// @lc code=start
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut sum = vec![0; 3];
        let mut max_sum = vec![0; 3];
        let mut max_sum_idx = vec![0; 3];
        let k = k as usize;
        for i in 2 * k..nums.len() {
            for j in 0..3 {
                sum[j] += nums[i - k * (2 - j)];
            }
            if i >= k * 3 - 1 {
                if sum[0] > max_sum[0] {
                    max_sum[0] = sum[0];
                    max_sum_idx[0] = i - k * 3 + 1;
                }
                if max_sum[0] + sum[1] > max_sum[1] {
                    max_sum[1] = max_sum[0] + sum[1];
                    max_sum_idx[1] = max_sum_idx[0];
                    max_sum_idx[2] = i - k * 2 + 1;
                }
                if max_sum[1] + sum[2] > max_sum[2] {
                    max_sum[2] = max_sum[1] + sum[2];
                    ans = vec![
                        max_sum_idx[1] as i32,
                        max_sum_idx[2] as i32,
                        (i - k + 1) as i32,
                    ];
                }
                for j in 0..3 {
                    sum[j] -= nums[i - k * (3 - j) + 1];
                }
            }
        }
        ans
    }
}
// @lc code=end
