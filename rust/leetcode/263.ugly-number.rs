/*
 * @lc app=leetcode id=263 lang=rust
 *
 * [263] Ugly Number
 */

// @lc code=start
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
         if n <= 0 {
            return false;
        }
        if n <= 6 {
            return true;
        }
        let ns = vec![2, 3, 5];
        for i in ns {
            if n % i == 0 {
                return Self::is_ugly(n / i);
            }
        }
        false
    }
}
// @lc code=end

