// # 3735. Lexicographically Smallest String After Reverse II 🔒

// Description
// -----------

// You are given a string `s` of length `n` consisting of lowercase English letters.

// You must perform **exactly** one operation by choosing any integer `k` such that `1 <= k <= n` and either:

// *   reverse the **first** `k` characters of `s`, or
// *   reverse the **last** `k` characters of `s`.

// Return the **lexicographically smallest** string that can be obtained after **exactly** one such operation.

// **Example 1:**

// **Input:** s = "dcab"

// **Output:** "acdb"

// **Explanation:**

// *   Choose `k = 3`, reverse the first 3 characters.
// *   Reverse `"dca"` to `"acd"`, resulting string `s = "acdb"`, which is the lexicographically smallest string achievable.

// **Example 2:**

// **Input:** s = "abba"

// **Output:** "aabb"

// **Explanation:**

// *   Choose `k = 3`, reverse the last 3 characters.
// *   Reverse `"bba"` to `"abb"`, so the resulting string is `"aabb"`, which is the lexicographically smallest string achievable.

// **Example 3:**

// **Input:** s = "zxy"

// **Output:** "xzy"

// **Explanation:**

// *   Choose `k = 2`, reverse the first 2 characters.
// *   Reverse `"zx"` to `"xz"`, so the resulting string is `"xzy"`, which is the lexicographically smallest string achievable.

// **Constraints:**

// *   `1 <= n == s.length <= 105`
// *   `s` consists of lowercase English letters.

// //   string lex_smallest(string s) {

// struct DoubleHash {
//     h1: Vec<u64>,
//     h2: Vec<u64>,
//     p1: Vec<u64>,
//     p2: Vec<u64>,
//     m1: u64,
//     m2: u64,
// }

// impl DoubleHash {
//     fn new(s: &[u8]) -> Self {
//         let n = s.len();
//         let (m1, m2) = (1_000_000_007, 1_000_000_009);
//         let (base1, base2) = (31, 37);
//         let (mut h1, mut h2) = (vec![0; n + 1], vec![0; n + 1]);
//         let (mut p1, mut p2) = (vec![1; n + 1], vec![1; n + 1]);

//         for i in 0..n {
//             h1[i + 1] = (h1[i] * base1 + (s[i] - b'a' + 1) as u64) % m1;
//             h2[i + 1] = (h2[i] * base2 + (s[i] - b'a' + 1) as u64) % m2;
//             p1[i + 1] = (p1[i] * base1) % m1;
//             p2[i + 1] = (p2[i] * base2) % m2;
//         }
//         Self { h1, h2, p1, p2, m1, m2 }
//     }

//     // 获取 [l, r) 的哈希值
//     fn get(&self, l: usize, r: usize) -> (u64, u64) {
//         let v1 = (self.h1[r] + self.m1 - (self.h1[l] * self.p1[r - l]) % self.m1) % self.m1;
//         let v2 = (self.h2[r] + self.m2 - (self.h2[l] * self.p2[r - l]) % self.m2) % self.m2;
//         (v1, v2)
//     }
// }

// impl Solution {
//     pub fn lex_smallest(s: String) -> String {
//         let n = s.len();
//         let s_bytes = s.as_bytes();
//         let r_bytes: Vec<u8> = s_bytes.iter().rev().cloned().collect();

//         let hash_s = DoubleHash::new(s_bytes);
//         let hash_r = DoubleHash::new(&r_bytes);

//         // 定义获取虚拟拼接字符串第 i 个字符和前缀哈希的闭包
//         // t_type: 0 为翻转前k个, 1 为翻转后k个
//         let get_char = |t_type: usize, k: usize, idx: usize| -> u8 {
//             if t_type == 0 { // rev(s[0..k]) + s[k..n] => r[n-k..n] + s[k..n]
//                 if idx < k { r_bytes[n - k + idx] } else { s_bytes[idx] }
//             } else { // s[0..n-k] + rev(s[n-k..n]) => s[0..n-k] + r[0..k]
//                 if idx < n - k { s_bytes[idx] } else { r_bytes[idx - (n - k)] }
//             }
//         };

//         let get_hash = |t_type: usize, k: usize, len: usize| -> (u64, u64) {
//             if t_type == 0 {
//                 if len <= k { hash_r.get(n - k, n - k + len) }
//                 else {
//                     let h1 = hash_r.get(n - k, n);
//                     let h2 = hash_s.get(k, len);
//                     let len2 = len - k;
//                     ((h1.0 * hash_s.p1[len2] + h2.0) % hash_s.m1, (h1.1 * hash_s.p2[len2] + h2.1) % hash_s.m2)
//                 }
//             } else {
//                 if len <= n - k { hash_s.get(0, len) }
//                 else {
//                     let h1 = hash_s.get(0, n - k);
//                     let h2 = hash_r.get(0, len - (n - k));
//                     let len2 = len - (n - k);
//                     ((h1.0 * hash_r.p1[len2] + h2.0) % hash_r.m1, (h1.1 * hash_r.p2[len2] + h2.1) % hash_r.m2)
//                 }
//             }
//         };

//         // 比较两组 (type, k) 生成的字符串
//         let is_smaller = |t1: usize, k1: usize, t2: usize, k2: usize| -> bool {
//             let mut low = 1;
//             let mut high = n;
//             let mut lcp = 0;
//             while low <= high {
//                 let mid = (low + high) / 2;
//                 if get_hash(t1, k1, mid) == get_hash(t2, k2, mid) {
//                     lcp = mid;
//                     low = mid + 1;
//                 } else {
//                     high = mid - 1;
//                 }
//             }
//             if lcp == n { false }
//             else { get_char(t1, k1, lcp) < get_char(t2, k2, lcp) }
//         };

//         let (mut best_t, mut best_k) = (0, 1);
//         for k in 1..=n {
//             for t in 0..2 {
//                 if is_smaller(t, k, best_t, best_k) {
//                     best_t = t;
//                     best_k = k;
//                 }
//             }
//         }

//         // 构建结果串
//         let mut res = Vec::with_capacity(n);
//         for i in 0..n { res.push(get_char(best_t, best_k, i)); }
//         String::from_utf8(res).unwrap()
//     }
// }

impl Solution {
    pub fn lex_smallest(s: String) -> String {
        const MOD: i64 = 1_000_000_007;
        const B: i64 = 29;
        let n = s.len();
        let mut prefix: Vec<_> = s
            .bytes()
            .scan(0, |pre, b| {
                *pre = (*pre * B + b as i64) % MOD;
                Some(*pre)
            })
            .collect();
        prefix.insert(0, 0);
        let mut suffix: Vec<_> = s
            .bytes()
            .rev()
            .scan(0, |pre, b| {
                *pre = (*pre * B + b as i64) % MOD;
                Some(*pre)
            })
            .collect();
        suffix.reverse();
        suffix.push(0);
        let mut base: Vec<_> = (0..n)
            .scan(1, |pre, b| {
                *pre = (*pre * B) % MOD;
                Some(*pre)
            })
            .collect();
        base.insert(0, 1);
        let get_prefix_hash = |l: usize, r: usize| {
            if l > r {
                return 0;
            }
            (prefix[r + 1] - prefix[l] * base[r - l + 1] % MOD + MOD) % MOD
        };
        let get_suffix_hash = |l: usize, r: usize| {
            if l > r {
                return 0;
            }
            (suffix[l] - suffix[r + 1] * base[r - l + 1] % MOD + MOD) % MOD
        };
        let get_total_hash = |k: usize, t: usize, l: usize| {
            if t == 0 {
                return if l <= k {
                    get_suffix_hash(k - l, k - 1)
                } else {
                    (get_suffix_hash(0, k - 1) * base[l - k] % MOD + get_prefix_hash(k, l - 1))
                        % MOD
                };
            }
            let nk = n - k;
            //  println!("{l},{nk},");
            if l <= nk {
                get_prefix_hash(0, l - 1)
            } else {
                (get_prefix_hash(0, (nk + n - 1) % n) * base[l - nk] % MOD
                    + get_suffix_hash(n - (l - nk), n - 1))
                    % MOD
            }
        };
        let get_char = |k: usize, t: usize, idx: usize| {
            if t == 0 {
                return if idx < k {
                    s.as_bytes()[k - 1 - idx]
                } else {
                    s.as_bytes()[idx]
                };
            }
            let nk = n - k;
            if idx < nk {
                s.as_bytes()[idx]
            } else {
                s.as_bytes()[(n - 1) - (idx - nk)]
            }
        };
        let is_less = |k: usize, i: usize, best_k: usize, best_i: usize| {
            let (mut l, mut r) = (0, n - 1);
            while l <= r {
                let m = l + (r - l) / 2;
                if get_total_hash(k, i, m + 1) == get_total_hash(best_k, best_i, m + 1) {
                    l = m + 1;
                } else {
                    if m == 0 {
                        break;
                    }
                    r = m - 1;
                }
            }
            l != n && get_char(k, i, l) < get_char(best_k, best_i, l)
        };
        let mn = s.bytes().min().unwrap();
        let (mut best_k, mut best_i) = (1, 0);
        for (k, b) in s.bytes().enumerate() {
            if b != mn {
                continue;
            }
            if is_less(k + 1, 0, best_k, best_i) {
                best_k = k + 1;
                best_i = 0;
            }
        }
        let last = s.as_bytes()[n - 1];
        for (k, b) in s.bytes().enumerate().rev() {
            if b < last {
                continue;
            }
            if is_less(k + 1, 1, best_k, best_i) {
                best_k = k + 1;
                best_i = 1;
            }
        }
        let mut ans = s.into_bytes();
        if best_i == 0 {
            ans[..best_k].reverse();
        } else {
            ans[n - best_k..].reverse();
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_lex_smallest_1() {
        assert_eq!(Solution::lex_smallest(String::from("dcab")), "acdb");
    }

    #[test]
    pub fn test_lex_smallest_2() {
        assert_eq!(Solution::lex_smallest(String::from("abba")), "aabb");
    }

    #[test]
    pub fn test_lex_smallest_3() {
        assert_eq!(Solution::lex_smallest(String::from("zxy")), "xzy");
    }
}
