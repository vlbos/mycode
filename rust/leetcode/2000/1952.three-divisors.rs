/*
 * @lc app=leetcode id=1952 lang=rust
 *
 * [1952] Three Divisors
 */

// @lc code=start
impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut cnt = 0;
        for i in 2..=n/2 {
            if n % i == 0 {
                cnt += 1;
                if cnt+2 > 3 {
                    return false;
                }
            }
        }
        cnt+2 == 3
    }
}
// @lc code=end
