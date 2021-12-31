/*
 * @lc app=leetcode id=695 lang=rust
 *
 * [695] Max Area of Island
 */

// @lc code=start
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid=grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        for i in 0..m{
            for j in 0..n{
                 if grid[i][j]!=1{
                 continue;
                 }
                 let mut area = 0;
                 dfs(&mut grid,i,j,&mut area);
                 ans = ans.max(area);
            }
        }
        fn dfs(grid: &mut Vec<Vec<i32>>,i : usize,j:usize,area:&mut i32){
               let m = grid.len() as i32;
                let n = grid[0].len() as i32;
                if grid[i][j]!=1{
                return ;
                }
                grid[i][j]=2;
                *area+=1;
                for ri in -1..=1
                    {
                    for ci in -1..=1
                    {
                        if ri==ci||ri+ci==0{
                        continue;
                        }
                        let nr = i as i32+ri;
                        let nc = j as i32+ci;
                        if nr>=0&& nr<m && nc>=0 && nc<n && grid[nr as usize][nc as usize]==1 {
                            dfs(grid,nr as usize,nc as usize,area);
                        }
                    }
                }

        }
        ans
    }
}
// @lc code=end

