/*
 * @lc app=leetcode id=862 lang=rust
 *
 * [862] Shortest Subarray with Sum at Least K
 */

// @lc code=start
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut p = vec![0; n + 1];
        for i in 0..n {
            p[i + 1] = p[i] + nums[i] as i64;
        }
        let mut ans = n + 1;
        let mut mono_q = std::collections::VecDeque::new();
        for i in 0..p.len() {
            while !mono_q.is_empty() && p[i] <= p[*mono_q.back().unwrap()] {
                mono_q.pop_back();
            }
            while !mono_q.is_empty() && p[i] >= p[*mono_q.front().unwrap()]+k  as i64{
                ans = ans.min(i - mono_q.pop_front().unwrap());
            }
            mono_q.push_back(i);
        }
        if ans < n + 1 {
            ans as _
        } else {
            -1
        }
    }
}
// @lc code=end
