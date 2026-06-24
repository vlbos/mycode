// 3916. Number of ZigZag Arrays III
// ## Description
// You are given three integers `n`, `l`, and `r`.
// A **ZigZag** array of length `n` is defined as follows:
// * Each element lies in the range `[l, r]`.
// * No **two** adjacent elements are equal.
// * No **three** consecutive elements form a **strictly increasing** or **strictly decreasing** sequence.
// Return the total number of valid **ZigZag** arrays.
// Since the answer may be large, return it **modulo** `109 + 7`.

// **Example 1:**
// **Input:** n = 3, l = 4, r = 5
// **Output:** 2
// **Explanation:**
// There are only 2 valid ZigZag arrays of length `n = 3` using values in the range `[4, 5]`:
// * `[4, 5, 4]`
// * `[5, 4, 5]`
// **Example 2:**
// **Input:** n = 3, l = 1, r = 3
// **Output:** 10
// **Explanation:**
// There are 10 valid ZigZag arrays of length `n = 3` using values in the range `[1, 3]`:
// * `[1, 2, 1]`, `[1, 3, 1]`, `[1, 3, 2]`
// * `[2, 1, 2]`, `[2, 1, 3]`, `[2, 3, 1]`, `[2, 3, 2]`
// * `[3, 1, 2]`, `[3, 1, 3]`, `[3, 2, 3]`
// All arrays meet the ZigZag conditions.

// **Constraints:**
// * `3 \<= n \<= 200`
// * `1 \<= l \< r \<= 10​​​​​​​9`

// int zig_zag_arrays(int n, int l, int r) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, mut r: i32) -> i32 {
        // const MOD:i64=1_000_000_007;
        // const SZ:usize=201;
        // use std::sync::OnceLock;
        // static INV_FACTORIAL:OnceLock<Vec<i64>>=OnceLock::new();
        // let inv_factorial=INV_FACTORIAL.get_or_init(||{
        //     let (mut inv,mut inv_fact)=(vec![1,1],vec![1,1]);
        //     while inv.len()<SZ{
        //         let n=inv.len() as i64;
        //         let v=inv[(MOD%n) as usize]*(MOD-MOD/n)%MOD;
        //         inv.push(v);
        //         let v=inv_fact[inv_fact.len()-1]*inv[inv.len()-1]%MOD;
        //         inv_fact.push(v);
        //     }
        //     inv_fact
        // });
        // let f=|x:usize|{
        //     let mut dp:Vec<_>=(0..x as i64).collect();
        //     for _ in 0..n-2{
        //         let mut new_dp=vec![0;x];
        //         for i in 0..x-1{
        //             new_dp[i+1]=(new_dp[i]+dp[x-1-i])%MOD;
        //         }
        //         dp=new_dp;
        //     }
        //     let total=dp[..x].iter().fold(0,|s,&v| (s+v)%MOD);
        //     (total+2)%MOD
        // };
        // let (m,n)=((r-l+1) as usize,n as usize);
        // if m<=n+1{
        // return f(m) as _}
        // let mut prefix=vec![0;n+1+1];
        // prefix[0]=1;
        // for i in 0..prefix.len()-1{
        //     prefix[i+1]=prefix[i]*(((m-1-i) as i64%MOD+MOD)%MOD)%MOD;
        // }
        // let mut suffix=vec![0;n+1+1];
        // *suffix.last_mut().unwrap()=1;
        // for i in (0..suffix.len()-1).rev(){
        //     suffix[i]=suffix[i+1]*(((m-1-i) as i64%MOD+MOD)%MOD)%MOD;
        // }
        // let mut ans=0;
        // for i in 0..=n{
        //     let ps=prefix[i] * suffix[i + 1] % MOD;
        //     let fps=f(i+1)*ps%MOD;
        //     let iif=inv_factorial[i] * inv_factorial[n - i] % MOD;
        //     let fi=(fps*iif)%MOD;
        //     let ni= if ((n - i)&1)==0{1} else {MOD - 1};
        //     let fnii=(fi*ni)%MOD;
        //     ans=(ans+fnii)%MOD;
        // }
        // ans as _

        const MOD: i64 = 1_000_000_007;

        let calc = |r: i64| {
            let mut f = vec![];
            let mut s = vec![];
            for v in 0..r + 1 {
                f.push(v);
                if s.is_empty() {
                    s.push(f[f.len() - 1])
                } else {
                    let x = s[s.len() - 1] + f[f.len() - 1];
                    s.push(x);
                }
            }

            for length in 3..n + 1 {
                for v in 0..r as usize + 1 {
                    if length % 2 == 0 {
                        if v == 0 {
                            f[v] = 0;
                        } else {
                            f[v] = s[v - 1];
                        }
                    } else {
                        f[v] = s[s.len() - 1] - s[v];
                        if f[v] < 0 {
                            f[v] += MOD;
                        }
                    }
                }
                s.clear();
                for v in 0..r as usize + 1 {
                    if s.is_empty() {
                        s.push(f[v]);
                    } else {
                        s.push(s[s.len() - 1] + f[v]);
                        if s[s.len() - 1] >= MOD {
                            *s.last_mut().unwrap() -= MOD
                        }
                    }
                }
            }
            let mut res = s[s.len() - 1] * 2;
            if res >= MOD {
                res -= MOD;
            }
            res
        };

        fn quick_pow(base: i64, p: i32) -> i64 {
            if p == 0 {
                return 1;
            }
            if p == 1 {
                return base;
            }

            let mut res = quick_pow(base, p / 2);
            res = res * res % MOD;
            if p % 2 == 1 {
                res = res * base % MOD;
            }
            res
        }

        r -= l;
        if r <= n {
            return calc(r as i64) as _;
        }

        let mut x = vec![];
        let mut y = vec![];
        for v in 0..n as i64 + 1 {
            x.push(v);
            y.push(calc(v));
        }

        let mut res = 0;
        for p in 0..n + 1 {
            let (mut a, mut b) = (1, 1);
            for index in 0..n + 1 {
                if p == index {
                    continue;
                }

                a = a * (r as i64 - x[index as usize]) % MOD;
                b = b * (x[p as usize] - x[index as usize]) % MOD;
            }

            a = a * y[p as usize] % MOD;

            res = (res + a * quick_pow(b, MOD as i32 - 2) % MOD) % MOD
        }

        res as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_zig_zag_arrays_1() {
        assert_eq!(2, Solution::zig_zag_arrays(3, 4, 5));
    }
    #[test]
    pub fn test_zig_zag_arrays_2() {
        assert_eq!(10, Solution::zig_zag_arrays(3, 1, 3));
    }
}
