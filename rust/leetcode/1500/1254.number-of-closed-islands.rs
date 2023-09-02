/*
 * @lc app=leetcode id=1254 lang=rust
 *
 * [1254] Number of Closed Islands
 */

// @lc code=start
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            if i < 0 || i >= m || j < 0 || j >= n {
                return 0;
            }
            if grid[i as usize][j as usize] == 1 {
                return 1;
            }
            grid[i as usize][j as usize] = 1;
              dfs(grid,i + 1,j) & dfs(grid,i - 1,j) & dfs(grid,i,j + 1) & dfs(grid,i,j - 1)
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        let mut grid = grid;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    ans += dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(grid.len(),grid[0].len());
        let mn=m*n;
        let mut parent:Vec<usize>=(0..=mn).collect();
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px]=py;
            }
        };
        for i in 0..m{
            for j in 0..n{
                if grid[i][j]==1{
                    continue
                }
                if i==0||i==m-1||j==0||j==n-1{
                    unite(i*n+j,mn,&mut parent);
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni<0||ni==m as i32||nj<0||nj==n as i32{
                        continue
                    }
                    let (ni,nj)=(ni as usize,nj as usize);
                    if grid[ni][nj]==0{
                        unite(i*n+j,ni*n+nj,&mut parent);
                    }
                }
            }
        }
        let pmn=find(mn,&mut parent);
        let mut ans=std::collections::HashSet::new();
        for i in 0..m{
            for j in 0..n{
                if grid[i][j]==0 {
                    let pij=find(i*n+j,&mut parent);
                    if pij!=pmn{
                        ans.insert(pij);
                    }
                }
            }
        }
        
        ans.len() as _
    }
}