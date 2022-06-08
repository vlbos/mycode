/*
 * @lc app=leetcode id=1425 lang=rust
 *
 * [1425] Constrained Subsequence Sum
 */

// @lc code=start
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
       let n = nums.len();
        let mut f = vec![0; n];
        f[0] = nums[0];
        let mut q = std::collections::VecDeque::from([0]);
        let mut ans = nums[0];
        for i in 1..n {
            while !q.is_empty() && i - *q.front().unwrap() > k as usize{
                q.pop_front();
            }
            f[i] = 0i32.max(f[*q.front().unwrap()]) + nums[i];
            ans = ans.max(f[i]);
            while !q.is_empty() && f[i] >= f[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(i);
        }
        ans
    }
}
// @lc code=end
