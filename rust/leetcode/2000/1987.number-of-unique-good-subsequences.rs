/*
 * @lc app=leetcode id=1987 lang=rust
 *
 * [1987] Number of Unique Good Subsequences
 */

// @lc code=start
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let (mut even, mut odd) = (0, 0);
        let p = 1_000_000_007;
        for ch in binary.chars() {
            if ch == '0' {
                even = (even + odd) % p;
            } else {
                odd = (even + odd + 1) % p;
            }
        }
        (even
            + odd
            + if binary.chars().find(|&c| c == '0').is_some() {
                1
            } else {
                0
            })
            % p
    }
}
// @lc code=end
