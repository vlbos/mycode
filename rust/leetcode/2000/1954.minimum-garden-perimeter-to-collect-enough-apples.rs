/*
 * @lc app=leetcode id=1954 lang=rust
 *
 * [1954] Minimum Garden Perimeter to Collect Enough Apples
 */

// @lc code=start
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let (mut l, mut r) = (1i64, 100000i64);
        let mut ans = 0;
        while l <= r {
            let mid = (l + r) / 2;
            if 2 * mid * (mid + 1) * (mid * 2 + 1) >= needed_apples {
                r = mid - 1;

                ans = mid;
            } else {
                l = mid + 1;
            }
        }
        ans*8
    }
}
// @lc code=end
