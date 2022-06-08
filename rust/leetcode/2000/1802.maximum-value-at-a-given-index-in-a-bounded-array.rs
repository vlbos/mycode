/*
 * @lc app=leetcode id=1802 lang=rust
 *
 * [1802] Maximum Value at a Given Index in a Bounded Array
 */

// @lc code=start
impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut ans = 1;
        let mut rest = max_sum - n;
        let (mut l, mut r) = (index, index);
        while l > 0 || r < n - 1 {
            let len = r - l + 1;
            if rest >= len {
                rest -= len;
                ans += 1;
                l = (l - 1).max(0);
                r = (r + 1).min(n - 1);
            } else {
                break;
            }
        }
        ans += rest / n;
        ans
    }
}
// @lc code=end
