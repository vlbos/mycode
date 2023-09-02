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
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m,n)=(grid.len(),grid[0].len());
        let mn=m*n;
        let mut parent:Vec<usize>=(0..mn).collect();
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
                if grid[i][j]=='0'{
                    continue
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni>=0 && ni<m as i32 && nj>=0 && nj<n as i32 && grid[ni as usize][nj as usize]=='1'{
                        unite(i*n+j,ni as usize*n+nj as usize,&mut parent);
                    }
                }
            }
        }
        let mut s=std::collections::HashSet::new();
        for i in 0..m{
            for j in 0..n{
                if grid[i][j]=='1'{
                    s.insert(find(i*n+j,&mut parent));
                }
            }}
            s.len() as _
    }
}