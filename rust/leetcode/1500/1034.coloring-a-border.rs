/*
 * @lc app=leetcode id=1034 lang=rust
 *
 * [1034] Coloring A Border
 */

// @lc code=start
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
         let old = grid[row as usize][col as usize];
           if old==color{
            return grid;
        }
        let  (n,m)=(grid.len(),grid[0].len());
        let mut vis=vec![vec![false;m];n];
        fn dfs(grid:&Vec<Vec<i32>>,vis:&mut Vec<Vec<bool>>,borders:&mut Vec<Vec<usize>>,row:i32,col:i32,old:i32){
            let dirs=[[0,1],[0,-1],[1,0],[-1,0]];
            let mut flag =false;
            vis[row as usize][col as usize]=true;
            for d in &dirs{
                let (i,j)=(row+d[0],col+d[1]);
                if i<0||i>=grid.len() as i32 ||j<0||j>=grid[0].len() as i32||grid[i as usize][j as usize]!=old{
                    flag=true;
                    continue;
                }
                if vis[i as usize][j as usize]{
                    continue;
                }
                dfs(grid,vis,borders,i,j,old);
            }
            if flag{
            borders.push(vec![row as usize,col as usize]);
            }
        }
        let mut boarders=Vec::new();
        dfs(&grid,&mut vis,&mut boarders,row,col,old);
        let mut grid = grid;
        for b in &boarders{
            grid[b[0]][b[1]]=color;
        }
        grid
    }
}
// @lc code=end

