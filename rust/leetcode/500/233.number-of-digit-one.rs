/*
 * @lc app=leetcode id=233 lang=rust
 *
 * [233] Number of Digit One
 */

// @lc code=start
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut mul_k = 1;
        let mut ans = 0;
        let n = n as i64;
        while n >= mul_k {
            ans += (n / (mul_k * 10)) * mul_k + 0i64.max((n % (mul_k * 10)) - mul_k + 1).min(mul_k);
            mul_k *= 10;
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let s=n.to_string();
        let bs=s.as_bytes();
        let len=s.len();
        if len==1 {
            return if n==0 {0}else{1}
        }
        let mut a=vec![0;len-1];
        a[0]=1;
        for i in 1..len-1{
            a[i]=a[i-1]*10+10i32.pow(i as u32);
        }
        let mut dp=vec![0;len];
        dp[0]=if bs[len-1]==b'0'{0}else{1};
        for i in 1..len{
            let cur=bs[len-i-1];
            if cur==b'0'{
                dp[i]=dp[i-1];
            }else if cur==b'1'{
                let rest=s[len-i..].parse::<i32>().unwrap()+1;
                dp[i]=rest+dp[i-1]+a[i-1];
            }else{
                dp[i]=(cur-b'0') as i32*a[i-1]+dp[i-1]+10i32.pow(i as u32);
            }
        }
        dp[len-1]
    }
}