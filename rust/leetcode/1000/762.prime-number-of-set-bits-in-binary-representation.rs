/*
 * @lc app=leetcode id=762 lang=rust
 *
 * [762] Prime Number of Set Bits in Binary Representation
 */

// @lc code=start
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
         let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut sum = 0;
        for n in left..=right {
            if primes.contains(&n.count_ones()) {
                sum += 1;
            }
        }
        sum
    }
}
// @lc code=end

