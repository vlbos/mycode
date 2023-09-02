/*
 * @lc app=leetcode id=1706 lang=rust
 *
 * [1706] Where Will the Ball Fall
 */

// @lc code=start
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        fn dfs(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i == grid.len() {
                return j as i32;
            }
            if j + 1 < grid[i].len() && grid[i][j] == 1 && grid[i][j + 1] == 1 {
                return dfs(grid, i + 1, j + 1);
            }
            if j > 0 && grid[i][j] == -1 && grid[i][j - 1] == -1 {
                return dfs(grid, i + 1, j - 1);
            }
            -1
        }
        for j in 0..grid[0].len() {
            ans.push(dfs(&grid, 0, j));
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m,n)=(grid.len(),grid[0].len());
        let mut dp:Vec<i32>=(0..n as i32).collect();
        for i in 0..m{
            for j in 0..n{
                let tmp=dp[j];
                if tmp==-1{
                    continue
                }
                let k=tmp as usize;
                if k<n-1 && grid[i][k]==1 && grid[i][k+1]==1{
                    dp[j]+=1;
                }else if k>0  && grid[i][k]==-1 && grid[i][k-1]==-1{
                    dp[j]-=1;
                }else{
                    dp[j]=-1;
                }
            }
        }
        dp
    }
}