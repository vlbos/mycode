/*
 * @lc app=leetcode id=668 lang=rust
 *
 * [668] Kth Smallest Number in Multiplication Table
 */

// @lc code=start
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let enough = |x: i32| {
            let mut ans = 0;
            for i in 1..=m {
                ans += n.min(x / i);
            }
            ans>=k
        };
        let (mut lo, mut hi) = (1, m * n);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if enough(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}
// @lc code=end
