/*
 * @lc app=leetcode id=1283 lang=rust
 *
 * [1283] Find the Smallest Divisor Given a Threshold
 */

// @lc code=start
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();
        let mut ans = -1;
        while l <= r {
            let mid = (l + r) / 2;
            let mut total = 0;
            for &n in &nums {
                total += (n - 1) / mid + 1;
            }
            if total <= threshold {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}
// @lc code=end
