/*
 * @lc app=leetcode id=1955 lang=rust
 *
 * [1955] Count Number of Special Subsequences
 */

// @lc code=start
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let mut f = vec![0; 3];
        let p = 1_000_000_007;
        for &num in &nums {
            let i = num as usize;
            f[i] = (f[i] * 2 % p + if i == 0 { 1 } else { f[i - 1] }) % p;
        }
        f[2]
    }
}
// @lc code=end
