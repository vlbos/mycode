/*
 * @lc app=leetcode id=799 lang=rust
 *
 * [799] Champagne Tower
 */

// @lc code=start
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![vec![0f64; 102]; 102];
        dp[0][0] = poured as f64;
        for i in 0..=query_row as usize {
            for j in 0..=i {
                let q = (dp[i][j] - 1.0) / 2.0;
                if q > 0.0 {
                    dp[i + 1][j] += q;
                    dp[i + 1][j + 1] += q;
                }
            }
        }
        dp[query_row as usize][query_glass as usize].min(1.0)
    }
}
// @lc code=end
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let (k,n,m)=(poured as f64,query_row as usize,query_glass as usize);
        let mut f=vec![vec![0.0;n+10];n+10];
        f[0][0]=k;
        for i in 0..=n{
            for j in 0..=i{
                if f[i][j]>1.0{
                    let d=(f[i][j]-1.0)/2.0;
                    f[i+1][j]+=d;
                    f[i+1][j+1]+=d;
                }
            }
        }
        f[n][m].min(1.0)
    }
}