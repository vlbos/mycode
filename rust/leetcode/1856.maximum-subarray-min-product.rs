/*
 * @lc app=leetcode id=1856 lang=rust
 *
 * [1856] Maximum Subarray Min-Product
 */

// @lc code=start
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut left, mut right) = (vec![0; n], vec![n - 1; n]);
        let mut s = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            while !s.is_empty() && nums[s[s.len() - 1]] >= num {
                right[s[s.len() - 1]] = i - 1;
                s.pop();
            }
            if !s.is_empty() {
                left[i] = s[s.len() - 1] + 1;
            }
            s.push(i);
        }
        let mut pre = vec![0];
        for (i, &num) in nums.iter().enumerate() {
            pre.push(pre[pre.len() - 1] + num as i64);
        }
        ((nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (pre[right[i] + 1] - pre[left[i]]) * num as i64)
            .max()
            .unwrap())
            % 1000_000_007) as _
    }
}
// @lc code=end
