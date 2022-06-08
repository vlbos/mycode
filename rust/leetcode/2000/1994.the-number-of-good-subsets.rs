/*
 * @lc app=leetcode id=1994 lang=rust
 *
 * [1994] The Number of Good Subsets
 */

// @lc code=start
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let num_max = 30;
        let p = 1_000_000_007;
        let mut freq = vec![0i64; num_max + 1];
        for &num in &nums {
            freq[num as usize] += 1;
        }
        let mut f = vec![0; 1 << primes.len()];
        f[0] = 1;
        for _ in 0..freq[1] {
            f[0] = f[0] * 2 % p;
        }
        for i in 2..=num_max {
            if freq[i] == 0 {
                continue;
            }
            let mut subset = 0;
            let mut x = i;
            let mut check = true;
            for (j, &prime) in primes.iter().enumerate() {
                if (x % (prime * prime) )== 0 {
                    check = false;
                    break;
                }
                if x % prime == 0 {
                    subset |= (1 << j);
                }
            }
            if !check {
                continue;
            }
            for mask in (1..(1 << primes.len())).rev() {
                if ( mask & subset) == subset {
                    f[mask] = (f[mask] + f[mask ^ subset]* freq[i])  % p;
                }
            }
        }
        let mut ans = 0;
        for mask in 1..(1 << primes.len()) {
            ans = (ans + f[mask]) % p;
        }
        ans as _
    }
}
// @lc code=end
