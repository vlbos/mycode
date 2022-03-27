/*
 * @lc app=leetcode id=1759 lang=rust
 *
 * [1759] Count Number of Homogenous Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let bs = s.as_bytes();
        let (mut l, mut r) = (0, 0);
        let mut ans = 0;
        while r < bs.len() {
            if bs[l] == bs[r] {
                r += 1;
            } else {
                let m = r - l;
                ans += m * (m + 1) / 2;
                ans %= 1000_000_007;
                l = r;
            }
        }
        let m = r - l;
        ans += m * (m + 1) / 2;
        ans %= 1000_000_007;
        ans as _
    }
}
// @lc code=end
