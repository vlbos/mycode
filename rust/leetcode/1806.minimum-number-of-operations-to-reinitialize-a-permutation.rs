/*
 * @lc app=leetcode id=1806 lang=rust
 *
 * [1806] Minimum Number of Operations to Reinitialize a Permutation
 */

// @lc code=start
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let mut ans = 1;
        let mut p = 2;
        while p != 1 {
            ans += 1;
            p <<= 1;
            p %= (n - 1);
        }
        ans
    }
}
// @lc code=end
