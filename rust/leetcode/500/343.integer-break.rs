/*
 * @lc app=leetcode id=343 lang=rust
 *
 * [343] Integer Break
 */

// @lc code=start
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n<=3{
        return n-1;
        }
        let q =( n/3) as u32;
        let r = n%3;
        if r==0{
            3i32.pow(q )
        }else if r==1{
            3i32.pow(q-1)*4
        }else{
        3i32.pow(q)*2}
    }
}
// @lc code=end

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n=n as usize;
        let mut dp=vec![0;n+1];
        for i in 2..=n{
            let mut max=0;
            for j in 1..i{
                max=max.max((j*(i-j)).max(j*dp[i-j]));
            }
            dp[i]=max;
        }
        dp[n] as _
    }
}


impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n<=3{
            return n-1
        }
        let n=n as usize;
        let mut dp=vec![0;n+1];
        dp[2]=1;
        for i in 3..=n{
            dp[i]=*[2*(i-2),2*dp[i-2],3*(i-3),3*dp[i-3]].into_iter().max().unwrap();
        }
        dp[n] as _
    }
}