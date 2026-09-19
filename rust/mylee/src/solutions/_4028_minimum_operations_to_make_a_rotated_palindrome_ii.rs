// 4028. Minimum Operations to Make a Rotated Palindrome II

// ## Description
// You are given a string `s` consisting of lowercase English letters.
// You can perform the following operations any number of times (including zero) and in any order:
// * **Increment**: Choose any index `i` and replace `s[i]` with the next lowercase English letter. The letter after `'z'` is `'a'`.
// * **Left rotate**: Move the first character of the string to the end.
// Return the **minimum** number of operations required to make `s` a palindrome.

// **Example 1:**
// **Input:** s = "abc"
// **Output:** 2
// **Explanation:**
// One optimal solution:
// * Left rotate the string: `"abc" -\> "bca"`.
// * Increment `'a'` to `'b'`: `"bca" -\> "bcb"`.
// * `"bcb"` is a palindrome. Thus, the answer is 2.
// **Example 2:**
// **Input:** s = "yb"
// **Output:** 3
// **Explanation:**
// * Increment the first character three times: `"yb" -\> "zb" -\> "ab" -\> "bb"`.
// * `"bb"` is a palindrome. Thus, the answer is 3.

// **Constraints:**
// * `2 \<= s.length \<= 5 \* 104`
// * `s​​​​​​​​​​​​​​` consists only of lowercase English letters.

// int min_operations(string s) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let bs = s.as_bytes();
        // ntt, convolution
        const MOD: i64 = 998244353;
        const G: i64 = 3;

        let powmod = |mut a: i64, mut b: i64| {
            a %= MOD;
            let mut result = 1;
            while b > 0 {
                if b & 1 == 1 {
                    result = result * a % MOD;
                }
                a = a * a % MOD;
                b >>= 1;
            }
            result
        };

        let ntt = |a: &mut Vec<i64>, invert: bool| {
            let n = a.len();
            let mut j = 0;
            for i in 1..n {
                let mut bit = n >> 1;
                while j & bit > 0 {
                    j ^= bit;
                    bit >>= 1;
                }
                j ^= bit;
                if i < j {
                    a.swap(i, j);
                }
            }
            let mut length = 2;
            while length <= n {
                let mut wlen = powmod(G, (MOD - 1) / length as i64);
                if invert {
                    wlen = powmod(wlen, MOD - 2);
                }
                let half = length >> 1;
                for i in (0..n).step_by(length) {
                    let mut w = 1;
                    for j in 0..half {
                        let u = a[i + j];
                        let v = a[i + j + half] * w % MOD;
                        let mut x = u + v;
                        if x >= MOD {
                            x -= MOD;
                        }
                        let mut y = u - v;
                        if y < 0 {
                            y += MOD;
                        }
                        a[i + j] = x;
                        a[i + j + half] = y;
                        w = w * wlen % MOD;
                    }
                }

                length <<= 1;
            }
            if invert {
                let inv_n = powmod(n as i64, MOD - 2);
                for i in 0..n {
                    a[i] = a[i] * inv_n % MOD;
                }
            }
        };

        let n = s.len();
        let mut sz = 1;
        while sz < 2 * n - 1 {
            sz <<= 1;
        }
        let mut cost = vec![0; n];
        for k in 0..13 {
            let mut a = vec![0; sz];
            let mut cnt = 0;
            for i in 0..n {
                let d = (((bs[i] - b'a') as i32 - k) + 26) % 26;
                if d >= 13 {
                    continue;
                }
                a[i] = 1;
                cnt += 1;
            }
            ntt(&mut a, false);
            for i in 0..a.len() {
                a[i] = a[i] * a[i] % MOD;
            }
            ntt(&mut a, true);
            for c in 0..n {
                cost[c] += cnt - (a[c] + if c + n < a.len() { a[c + n] } else { 0 });
            }
        }
        let mut result = i64::MAX;
        for i in 0..n {
            result = result.min(i as i64 + cost[(2 * i + n - 1) % n]);
        }
        result as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(Solution::min_operations(String::from("abc")), 2);
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(Solution::min_operations(String::from("yb")), 3);
    }
}
