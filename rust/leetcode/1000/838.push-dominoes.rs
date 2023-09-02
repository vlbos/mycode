/*
 * @lc app=leetcode id=838 lang=rust
 *
 * [838] Push Dominoes
 */

// @lc code=start
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let mut forces = vec![0; n];
        let mut force = 0;
        for (i, v) in dominoes.chars().enumerate() {
            if v == 'R' {
                force = n as i32;
            } else if v == 'L' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[i] += force;
        }

        force = 0;
        for (i, v) in dominoes.chars().rev().enumerate() {
            if v == 'L' {
                force = n as i32;
            } else if v == 'R' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[n - i - 1] -= force;
        }
        let mut ans = String::new();
        for f in &forces {
            ans.push(if *f > 0 {
                'R'
            } else if *f < 0 {
                'L'
            } else {
                '.'
            });
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n=dominoes.len();
        let mut dp=vec![0;n];
        for (i,c) in dominoes.chars().enumerate(){
            if c=='L'{
                dp[i]=-1;
            }else if c=='R'{
                dp[i]=1;
            }
        }
        for i in 0..n{
            if dp[i]>0 && i+1<n && dp[i+1]==0{
                dp[i+1]=dp[i]+1;
            }
            let mut j=i;
            while dp[j]<0 && j>0 && (dp[j-1]==0||dp[j]+dp[j-1]>0){
                if dp[j]+dp[j-1]==1{
                    dp[j-1]=0;
                    break
                }
                dp[j-1]=dp[j]-1;
                j-=1;
            }
        }
        dp.into_iter().map(|x| if x>0{'R'}else if x<0{'L'}else{'.'}).collect()
    }
}