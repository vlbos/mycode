/*
 * @lc app=leetcode id=1524 lang=rust
 *
 * [1524] Number of Sub-arrays With Odd Sum
 */

// @lc code=start
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (1, 0);
        let mut ans = 0;
        let mut sum = 0;
        for &v in &arr {
            sum += v;
            ans += if sum % 2 == 0 { odd } else { even };
            ans %= 1000_000_007;
            if sum % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        ans
    }
}
// @lc code=end
