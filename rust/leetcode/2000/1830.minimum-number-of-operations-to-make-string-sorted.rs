/*
 * @lc app=leetcode id=1830 lang=rust
 *
 * [1830] Minimum Number of Operations to Make String Sorted
 */

// @lc code=start
impl Solution {
    pub fn make_string_sorted(s: String) -> i32 {
       let n = s.len();
        let mut fac = vec![0; n + 1];
        let mut facinv = vec![0; n + 1];
        fac[0] = 1;
        facinv[0] = 1;
        let p = 1_000_000_007;
        let quick_mul = |x: i32, mut y: i32| {
            let mut ans = 1;
            let mut mul = x as i64;
            while y > 0 {
                if y & 1 > 0 {
                    ans = ans  * mul % p;
                }
                mul = mul * mul % p;
                y >>= 1;
            }
            ans as i32
        };
        for i in 1..n {
            fac[i] = ((fac[i - 1] as i64) * (i as i64) % p) as i32;
            facinv[i] = quick_mul(fac[i], p as i32- 2);
        }
        let mut freq = vec![0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for (i, b) in s[..s.len()-1].bytes().enumerate() {
            let rank = freq[..(b - b'a') as usize].iter().sum::<i32>();
            let mut cur = (rank as i64) * (fac[n - i - 1] as i64) % p;
            for j in 0..26 {
                cur = cur * (facinv[freq[j] as usize] as i64) % p;
            }
            ans = (ans + cur) % p;
            freq[(b - b'a') as usize]-=1;
        }
        ans as _
    }
}
// @lc code=end
