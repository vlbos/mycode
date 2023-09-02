/*
 * @lc app=leetcode id=650 lang=rust
 *
 * [650] 2 Keys Keyboard
 */

// @lc code=start
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp =vec![0;n +1];
        for i in 2..=n{
            dp[i]=i32::MAX;
            let mut j = 1;
            while j*j<=i{
                if i%j==0{
                    dp[i]=dp[i].min(dp[j]+(i/j) as i32);
                    dp[i]=dp[i].min(dp[i/j]+j as i32);
                }
                j+=1;
            }
        }
        dp[n]
    }
}
// @lc code=end
impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let mut ans=0;
        let mut i=2;
        while i*i<=n{
            while n%i==0{
                n/=i;
                ans+=i;
            }
            i+=1;
        }
        if n>1{
            ans+=n;
        }
        ans

    }
}
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n=n as usize;
        let mut f=vec![0;n+1];
        for i in 2..=n{
            f[i]=i32::MAX as usize/2;
            let mut j=1;
            while j*j<=i{
                if i%j==0{
                    f[i]=f[i].min(f[j]+i/j);
                    f[i]=f[i].min(f[i/j]+j);
                }
                j+=1;
            }
        }
        f[n] as _
    }
}