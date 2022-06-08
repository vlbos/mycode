/*
 * @lc app=leetcode id=808 lang=rust
 *
 * [808] Soup Servings
 */

// @lc code=start
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let n = n/25+ if n%25>0 {1}else{0};
        if n>=500{
        return 1f64;
        }
        let n1 = n as usize+1;
        let mut dp =vec![vec![0f64;n1];n1];
        let m = |i:i32|->usize{i.max(0) as usize};
        for s in 0..=2*n{
        for i in 0..=n{
                let  j= s-i;
                if j<0||j>n{
                    continue;
                }
                let mut ans = 0.0;
                if i==0{
                    ans=1.0;
                }
                if i==0 && j==0 {
                ans = 0.5;
                }
                if i>0 && j>0{
                    ans=0.25*(dp[m(i-4)][j as usize]+dp[m(i-3)][m(j-1)]+dp[m(i-2)][m(j-2)]+dp[m(i-1)][m(j-3)]);
                }
                dp[i as usize][j as usize]=ans;
        }}
        dp[n as usize][n as usize]
    }
}
// @lc code=end

