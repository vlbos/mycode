/*
 * @lc app=leetcode id=1020 lang=rust
 *
 * [1020] Number of Enclaves
 */

// @lc code=start
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) {
            if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
                return;
            }
            if grid[i as usize][j as usize] != 1 {
                return;
            }
            grid[i  as usize][j  as usize] = 0;
            let dirs = [[0, -1], [0, 1], [1, 0], [-1, 0]];
            for d in &dirs {
                dfs(grid, i + d[0], j + d[1]);
            }
        }
        let n =grid.len();
        let m =grid[0].len();
        for i in 0..n{
            if grid[i][0] == 1 {
                dfs(&mut grid, i as i32, 0);
            }
            if grid[i][m-1] == 1 {
                dfs(&mut grid, i  as i32, m as i32-1);
            }
        }
        for j in 0..m {
            if grid[0][j] == 1 {
                dfs(&mut grid, 0, j  as i32);
            }
            if grid[n-1][j] == 1 {
                dfs(&mut grid, n as i32-1, j  as i32);
            }
        }
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    ans += 1;
                }
            }
        }
        ans 
    }
}
// @lc code=end
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
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
                if grid[i][j]==0{continue}
                if i==0 ||  i ==m-1 || j ==0 || j==n-1 {
                    unite(i*n+j,mn,&mut parent);
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni<0 ||  ni ==m as i32 || nj <0 || nj==n as i32 {
                       continue
                    }
                    let (ni,nj)=(ni as usize,nj as usize);
                    if grid[ni][nj]==1{
                        unite(i*n+j,ni*n+nj,&mut parent);
                    }
                }
            }
        }
        let mut ans=0;
        let pmn=find(mn,&mut parent);
         for i in 0..m{
            for j in 0..n{
                if grid[i][j]==1 && find(n*i+j,&mut parent)!=pmn{
                        ans+=1;
                }
            }
        }
        ans
    }
}