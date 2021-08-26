/*
 * @lc app=leetcode.cn id=868 lang=rust
 *
 * [868] 二进制间距
 */

// @lc code=start
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut m = 0;
        let mut n = n;
        let mut i = i32::MAX;
        let mut c = 0;
        while n > 0 {
            if n % 2 == 1 {
                if i != i32::MAX && (c - i) > m {
                    m = c - i;
                }
                i = c;
            }
            c += 1;
            n /= 2;
        }
        m
    }
}
// @lc code=end
