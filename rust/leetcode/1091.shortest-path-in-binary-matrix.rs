/*
 * @lc app=leetcode id=1091 lang=rust
 *
 * [1091] Shortest Path in Binary Matrix
 */

// @lc code=start
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n =grid.len();
        if grid[0][0]==1||grid[n-1][n-1]==1{
        return -1;
        }
        let mut dp =vec![vec![i32::MAX;n];n];
        let get_grid=|i:usize,j:usize|->Vec<(usize,usize)>{
            let n = n as i32;
            let mut ans = Vec::new();
            let (x,y) = (i as i32,j as i32);
            for  dx in -1..=1{
                 for  dy in -1..=1{
                  if dx==0&& dy==0{
                    continue;
                  }
                  let (nx,ny)=(x+dx,y+dy);
                  if nx<0||nx>=n||ny<0||ny>=n{
                  continue;
                  }
                  if grid[nx as usize][ny as usize]!=0{
                 continue;
                 }
                 ans.push((nx as usize,ny as usize));
                 }
            }
            ans
        };
        dp[0][0]=1;
        let mut q = std::collections::VecDeque::new();
        q.push_back((0,0));
        while let Some((r,c))=q.pop_front(){
             for (nr,nc) in get_grid(r,c){
                if dp[nr][nc]==i32::MAX{
                 q.push_back((nr,nc));
                }
                dp[nr][nc]=dp[nr][nc].min(dp[r][c]+1);
             }
        }
        if dp[n-1][n-1]==i32::MAX {-1}else{dp[n-1][n-1]}
    }
}
// @lc code=end

