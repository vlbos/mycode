/*
 * @lc app=leetcode id=1799 lang=rust
 *
 * [1799] Maximize Score After N Operations
 */

// @lc code=start
impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let n1 = 1 << n;
        let mut dp = vec![0; n1];
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        for i in 0..n1 {
            let c = i.count_ones() as i32;
            if c % 2 > 0 {
                continue;
            }
            for j in 0..n {
                if i & (1 << j) == 0 {
                    continue;
                }
                for k in j + 1..n {
                    if i & (1 << k) > 0 {
                        dp[i] =
                            dp[i].max(dp[i ^ (1 << j) ^ (1 << k)] + c / 2 * gcd(nums[j], nums[k]));
                    }
                }
            }
        }
        dp[n1 - 1]
    }
}
// @lc code=end
