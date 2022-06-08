/*
 * @lc app=leetcode id=891 lang=rust
 *
 * [891] Sum of Subsequence Widths
 */

// @lc code=start
impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut pow2 = vec![0; n];
        pow2[0] = 1;
        let p = 1_000_000_007;
        for i in 1..n {
            pow2[i] = pow2[i - 1] * 2 % p;
        }
        let mut ans = 0;
        for i in 0..n {
            ans = (ans + (pow2[i] - pow2[n - i - 1]) * nums[i] as i64) % p;
        }
        ans as _
    }
}
// @lc code=end
