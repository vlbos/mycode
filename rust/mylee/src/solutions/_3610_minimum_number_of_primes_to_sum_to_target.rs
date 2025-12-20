// ## [3610\. Minimum Number of Primes to Sum to Target 🔒](https://leetcode.com/problems/minimum-number-of-primes-to-sum-to-target)

// ## Description

// You are given two integers `n` and `m`.

// You have to select a multiset of **prime numbers** from the **first** `m` prime numbers such that the sum of the selected primes is **exactly** `n`.
//  You may use each prime number **multiple** times.

// Return the **minimum** number of prime numbers needed to sum up to `n`, or -1 if it is not possible.

// **Example 1:**

// **Input:** n = 10, m = 2

// **Output:** 4

// **Explanation:**

// The first 2 primes are \[2, 3\]. The sum 10 can be formed as 2 + 2 + 3 + 3, requiring 4 primes.

// **Example 2:**

// **Input:** n = 15, m = 5

// **Output:** 3

// **Explanation:**

// The first 5 primes are \[2, 3, 5, 7, 11\]. The sum 15 can be formed as 5 + 5 + 5, requiring 3 primes.

// **Example 3:**

// **Input:** n = 7, m = 6

// **Output:** 1

// **Explanation:**

// The first 6 primes are \[2, 3, 5, 7, 11, 13\]. The sum 7 can be formed directly by prime 7, requiring only 1 prime.

// **Constraints:**

// +   `1 <= n <= 1000`
// +   `1 <= m <= 1000`

//  int min_number_of_primes(int n, int m) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_number_of_primes(n: i32, m: i32) -> i32 {
        use std::sync::OnceLock;
        static prime_s: OnceLock<Vec<i32>> = OnceLock::new();
        let primes = prime_s.get_or_init(|| {
            let mut prime = vec![];
            for x in 2.. {
                let mut is_prime = true;
                for &p in &prime {
                    if p * p > x {
                        break;
                    }
                    if x % p == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    prime.push(x);
                    if prime.len() == 1000 {
                        break;
                    }
                }
            }
            prime
        });
        let n = n as usize;
        let mut f = vec![i32::MAX / 2; n + 1];
        f[0] = 0;
        for &x in &primes[..m as usize] {
            for i in x as usize..=n {
                f[i] = f[i].min(f[i - x as usize] + 1);
            }
        }
        if f[n] == i32::MAX / 2 { -1 } else { f[n] }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_number_of_primes_1() {
        assert_eq!(4, Solution::min_number_of_primes(10, 2));
    }
    #[test]
    pub fn test_min_number_of_primes_2() {
        assert_eq!(3, Solution::min_number_of_primes(15, 5));
    }
    #[test]
    pub fn test_min_number_of_primes_3() {
        assert_eq!(1, Solution::min_number_of_primes(7, 6));
    }
}
