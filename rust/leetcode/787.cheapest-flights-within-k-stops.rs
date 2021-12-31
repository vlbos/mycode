/*
 * @lc app=leetcode id=787 lang=rust
 *
 * [787] Cheapest Flights Within K Stops
 */

// @lc code=start
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let inf = 10000*101+1;
        let mut dp=vec![inf;n as usize];
        dp[src as usize]=0;
        let mut ans= inf;
        for t in 1..=k as usize +1{
                let mut dp2=vec![inf;n as usize];
                for f in &flights{
                     let j = f[0] as usize;
                     let i = f[1] as usize;
                     let c = f[2] ;
                     dp2[i]=dp2[i].min(dp[j]+c);
                }
                dp=dp2;
                ans=ans.min(dp[dst as usize]);
        }
        if ans==inf {-1}else{ans}
    }
}
// @lc code=end

