/*
 * @lc app=leetcode id=1837 lang=rust
 *
 * [1837] Sum of Digits in Base K
 */

// @lc code=start
impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut nn = n;
        let mut sum = 0;
        while nn > 0 {
            sum += nn % k;
            nn /= k;
        }
        sum
    }
}
// @lc code=end
