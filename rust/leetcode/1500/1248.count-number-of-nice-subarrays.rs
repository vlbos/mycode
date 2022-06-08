/*
 * @lc app=leetcode id=1248 lang=rust
 *
 * [1248] Count Number of Nice Subarrays
 */

// @lc code=start
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut pre = vec![0; n + 1];
        let mut odd = 0;
        let mut ans = 0;
        pre[0] = 1;
        for &v in &nums {
            odd += v & 1;
            ans += if odd >= k { pre[(odd - k) as usize] } else { 0 };
            pre[odd as usize] += 1;
        }
        ans
    }
}
// @lc code=end
