/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] 快乐数
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut s = HashSet::new();
        s.insert(n);
        let mut r = 0;
        let mut n = n;
        loop {
            while n > 0 {
                r += (n % 10) * (n % 10);
                n /= 10;
            }
            if s.contains(&r) {
                return false;
            }
            if r == 1 {
                break;
            }
            s.insert(r);
            n = r;
            r = 0;
        }
        true
    }
}
// @lc code=end
