/*
 * @lc app=leetcode id=526 lang=rust
 *
 * [526] Beautiful Arrangement
 */

// @lc code=start
impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n1 = 1<<n as u32;
        let mut dp = vec![0;n1 as usize];
        dp[0]=1;
        for j in 1..n1{
            let b1=(j as i32).count_ones() as i32;
            for i in 0..n{
                if j&(1<<i)!=0 &&(b1%(i+1)==0||(i+1)%b1==0){
                    dp[j]+=dp[j^(1<<i)];
                }
            }
        }
        dp[n1 as usize-1]
    }
}
// @lc code=end

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n=n as usize;
        let mut vis=vec![false;n+1];
        let mut matches=vec![vec![];n+1];
        for i in 1..=n{
            for j in 1..=n{
                if i%j==0||j%i==0{
                    matches[i].push(j);
                }
            }
        }
        fn back_track(i:usize,vis:&mut Vec<bool>,matches:&Vec<Vec<usize>>,ans:&mut i32){
            if i==vis.len(){
                *ans+=1;
                return
            }
            for &j in &matches[i]{
                if !vis[j]{
                    vis[j]=true;
                     back_track(i+1,vis,matches,ans);
                    vis[j]=false;
                }
            }
        }
        let mut ans=0;
        back_track(1,&mut vis,&matches,&mut ans);
        ans
    }
}