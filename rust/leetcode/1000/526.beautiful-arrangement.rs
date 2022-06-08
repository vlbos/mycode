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

