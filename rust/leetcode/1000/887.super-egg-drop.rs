/*
 * @lc app=leetcode id=887 lang=rust
 *
 * [887] Super Egg Drop
 */

// @lc code=start
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        use std::collections::HashMap;
        let mut memo = HashMap::new();
        fn dp(k: i32, n: i32, memo: &mut HashMap<(i32,i32), i32>) -> i32 {
            if let Some(&f) = memo.get(&(n, k)) {
                return f;
            }
            let mut ans = 0;
            if n == 0 {
                ans = 0;
            } else if k == 1 {
                ans = n;
            } else {
                let (mut lo, mut hi) = (1, n);
                while lo + 1 < hi {
                    let x = (lo + hi) / 2;
                    let t1 = dp(k - 1, x - 1,memo);
                    let t2 = dp(k, n - x,memo);
                    if t1 < t2 {
                        lo = x;
                    } else if t1 > t2 {
                        hi = x;
                    } else {
                        lo = x;
                        hi = x;
                    }
                }
                ans = 1 + dp(k - 1, lo - 1,memo)
                    .max(dp(k, n - lo,memo))
                    .min(dp(k - 1, hi - 1,memo).max(dp(k, n - hi,memo)));
            }
            memo.insert((n, k), ans);
            ans
        }
        dp(k, n, &mut memo)
    }
}
// @lc code=end
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let n=n as usize;
        let mut dp:Vec<usize>=(0..=n).collect();
        for _ in 2..k+1{
            let mut dp2=vec![0;n+1];
            let mut x=1;
            for m in 1..=n{
                while x<m && dp[x-1].max(dp2[m-x])>=dp[x].max(dp2[m-x-1]){
                    x+=1;
                }
                dp2[m]=dp[x-1].max(dp2[m-x])+1;
            }
            dp=dp2;
        }
        
        dp[n] as _

    }
}