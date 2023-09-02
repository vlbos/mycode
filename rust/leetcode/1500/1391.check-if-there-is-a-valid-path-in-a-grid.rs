/*
 * @lc app=leetcode id=1391 lang=rust
 *
 * [1391] Check if There is a Valid Path in a Grid
 */

// @lc code=start
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
         let (m, n) = (grid.len(), grid[0].len());
        if m == 1 && n == 1 {
            return true;
        }
        let mut visited = vec![vec![false;n];m];
        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>,i: usize, j: usize) -> bool {
            let (m, n) = (grid.len(), grid[0].len());
            if i == m - 1 && j == n - 1 {
                return true;
            }
            visited[i][j]=true;
            let (mi, ni) = (m as i32, n as i32);
            let patterns = [0b0101, 0b1010, 0b0011, 0b0110, 0b1001, 0b1100];
            let dirs = [[0, -1], [1, 0], [0, 1], [-1, 0]];
            for (k, d) in dirs.iter().enumerate() {
                let p = patterns[grid[i][j] as usize - 1];
                if p & (1 << k) > 0 {
                    let (ii, jj) = (i as i32 + d[0], j as i32 + d[1]);
                    if ii >= 0
                        && ii < mi
                        && jj >= 0
                        && jj < ni
                        && !visited[ii as usize][jj as usize] && patterns[grid[ii as usize][jj as usize] as usize - 1] & (1 << ((k + 2) % 4)) > 0
                    {
                        if dfs(grid, visited,ii as usize, jj as usize) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        dfs(&grid,&mut visited,0, 0)
    }
}
// @lc code=end
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let (m,n)=(grid.len() as i32,grid[0].len() as i32);
        let mut parent:Vec<i32>=(0..m*n).collect();
        fn find(x:i32,parent:&mut Vec<i32>)->i32{
            let px=parent[x as usize];
            if px!=x{
                parent[x as usize]=find(px,parent);
            }
            parent[x as usize]
        }
        let unite=|x:i32,y:i32,parent:&mut Vec<i32>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px as usize]=py;
            }
        };
        let patterns=[0,0b1010,0b0101,0b1100,0b0110,0b1001,0b0011];
        for (i,row) in grid.iter().enumerate(){
            for (j,&v ) in row.iter().enumerate(){
                let (i,j)=(i as i32,j as i32);
                let p =patterns[v as usize];
                 for (k,d) in [-1,0,1,0,-1].windows(2).enumerate(){
                     if  p&(1<<k)==0{
                         continue
                     }
                     let (ni,nj)=(i+d[0],j+d[1]);
                     if ni>=0 && ni<m && nj>=0 && nj<n && patterns[grid[ni as usize][nj as usize] as usize]&(1<<((k+2)%4))!=0{
                         unite(i*n+j,ni*n+nj,&mut parent);
                     }
                 }
            }
        }
        find(0,&mut parent)==find(m*n-1,&mut parent)
    }
}