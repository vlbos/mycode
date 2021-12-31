/*
 * @lc app=leetcode id=866 lang=rust
 *
 * [866] Prime Palindrome
 */

// @lc code=start
impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        let is_prime = |n: i32| -> bool {
            if n < 2 {
                return false;
            }
            let r = (n as f32).sqrt() as i32;
            for i in 2..=r {
                if n % i == 0 {
                    return false;
                }
            }
            true
        };
        let is_palindrome = |n: i32| -> bool {
            let n = n as i64;
            n.to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i64>()
                .unwrap() == n
            
        };
        let mut n = n;
        loop {
            if is_palindrome(n) && is_prime(n) {
                return n;
            }
            n += 1;
            if n > 10_000_000 && n < 100_000_000 {
                n = 100_000_000;
            }
        }
        0
    }
}
// @lc code=end
