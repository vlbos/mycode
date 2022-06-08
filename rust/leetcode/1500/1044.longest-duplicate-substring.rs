/*
 * @lc app=leetcode id=1044 lang=rust
 *
 * [1044] Longest Duplicate Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
          use rand::Rng;
        let mut rng = rand::thread_rng();
        let a1: i64 = rng.gen_range(27, 100);
        let a2: i64 = rng.gen_range(27, 100);
        let p = 1_000_000_007;
        let mod1: i64 = rng.gen_range(p, i32::MAX) as i64;
        let mod2: i64 = rng.gen_range(p, i32::MAX) as i64;
        let arr: Vec<i32> = s.bytes().map(|x| (x - b'a') as i32).collect();
                let n = s.len();

        let pow_radix = |a: i64, mut m: i32, mut mm: i64| -> i64 {
            let mut ans = 1;
            let (mut c, mut m) = (a , m as i64);
            while m > 0 {
                if m % 2 == 1 {
                    ans = ans * c % mm;
                    if ans < 0 {
                        ans += mm;
                    }
                }
                c = c * c % mm;
                if c < 0 {
                    c += mm;
                }
                m /= 2;
            }

            ans
        };
        let check = |m: i32| -> i32 {
            let al1 = pow_radix(a1, m, mod1) ;
            let al2 = pow_radix(a2, m, mod2);
            let (mut h1, mut h2) = (0, 0);
                        let m = m as usize;
            for i in 0..m {
                h1 = (h1 * a1 % mod1 + arr[i] as i64) % mod1;
                h2 = (h2 * a2 % mod2 + arr[i]  as i64) % mod2;

                if h1 < 0 {
                    h1 += mod1;
                }
                if h2 < 0 {
                    h2 += mod2;
                }
            }
            let mut seen = std::collections::HashSet::new();
            seen.insert((h1, h2));
            for i in 1..=n - m {
                h1 = (h1 * a1 % mod1 - ((arr[i-1] as i64) * al1 % mod1) + (arr[i + m - 1] as i64)) % mod1;
                if h1 < 0 {
                    h1 += mod1;
                }
                h2 = (h2 * a2 % mod2 - ((arr[i-1] as i64) * al2 % mod2) + arr[i + m - 1] as i64) % mod2;
                if h2 < 0 {
                    h2 += mod2;
                }
                if seen.contains(&(h1, h2)) {
                    return i as i32;
                }
                seen.insert((h1, h2));
            }
            -1
        };
       
        let (mut l, mut r) = (1, n as i32 - 1);
        let mut length = 0;
        let mut start = -1;
        while l <= r {
            let m = l+( r-l+1) / 2;
            let idx = check(m);
            if idx != -1 {
                l = m + 1;
                length = m;
                start = idx;
            } else {
                r = m - 1;
            }
        }
        if start == -1 {
            String::new()
        } else {
            s[start as usize..(start + length) as usize].to_string()
        }
    }
}
// @lc code=end
