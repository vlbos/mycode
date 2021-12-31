/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let mut grid = grid;
        fn dfs(grid: &mut Vec<Vec<char>>,i:usize,j:usize){
            if grid[i][j]=='1'{
                grid[i][j]='2';
                if i+1<grid.len(){
                    dfs(grid,i+1,j);
                }
                if j+1<grid[i].len(){
                    dfs(grid,i,j+1);
                }
                if i>0{
                    dfs(grid,i-1,j);
                }
                if j>0{
                    dfs(grid,i,j-1);
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j]=='1'{
                    dfs(&mut grid,i,j);
                    ans+=1;
                }
            }
        }
        ans
    }
}
// @lc code=end
