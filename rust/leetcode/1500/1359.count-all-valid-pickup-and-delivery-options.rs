/*
 * @lc app=leetcode id=1359 lang=rust
 *
 * [1359] Count All Valid Pickup and Delivery Options
 */

// @lc code=start
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut ans = 1;
        let n = n as i64;
        let p = 1_000_000_007;
        for i in 2..=n {
            ans = ans * (i * 2 - 1) % p * i % p;
        }
        ans as _
    }
}
// @lc code=end
