/*
 * @lc app=leetcode id=313 lang=rust
 *
 * [313] Super Ugly Number
 */

// @lc code=start
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        if n==1{
        return 1;
        }
        let n = n as usize;
        let m = primes.len();
        let mut p = vec![0;m];
        let mut nums = vec![1;m];
        let mut dp = vec![0;n+1];
        for i in 1..=n{
             let min = *(nums.iter().min().unwrap());
             dp[i]=min;
             for j in 0..m{
                if nums[j]==min{
                    p[j]+=1;
                    nums[j]=dp[p[j]]*primes[j];
                }
             } 
        }
        dp[n]
    }
}
// @lc code=end

