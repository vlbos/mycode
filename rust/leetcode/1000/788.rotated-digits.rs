/*
 * @lc app=leetcode id=788 lang=rust
 *
 * [788] Rotated Digits
 */

// @lc code=start
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut d: Vec<i32> = vec![0, 0, 1, -1, -1, 1, 1, -1, 0, 1];
        d.extend(vec![0; (n - 9).max(0) as usize]);
        let mut ans = 0;
        for i in 0..=n as usize {
            d[i] = if d[i / 10] == -1 || d[i % 10] == -1 {-1} else {d[i / 10] | d[i % 10]};
            ans += if d[i] == 1 {1} else {0};
        }
        ans
    }
}
// @lc code=end

