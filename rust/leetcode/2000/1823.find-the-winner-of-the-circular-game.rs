/*
 * @lc app=leetcode id=1823 lang=rust
 *
 * [1823] Find the Winner of the Circular Game
 */

// @lc code=start
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut ans = Self::find_the_winner(n - 1,k) + k;
        ans %= n;
        if ans == 0 {
            n
        } else {
            ans
        }
    }
}
// @lc code=end
