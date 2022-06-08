/*
 * @lc app=leetcode id=1551 lang=rust
 *
 * [1551] Minimum Operations to Make Array Equal
 */

// @lc code=start
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n {
            let x = 2 * i + 1;
            if x > n {
                ans += x - n;
            }
        }
        ans
    }
}
// @lc code=end
