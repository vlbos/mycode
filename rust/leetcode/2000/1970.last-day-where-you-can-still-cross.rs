/*
 * @lc app=leetcode id=1970 lang=rust
 *
 * [1970] Last Day Where You Can Still Cross
 */

// @lc code=start
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (mut left, mut right) = (0, row * col);
        let dirs = [0, 1, 0, -1, 0];
        while left <= right {
            let mid = (left + right) / 2;
            let mut grid = vec![vec![1; col as usize]; row as usize];
            for cell in &cells[..mid as usize] {
                let (r, c) = (cell[0] as usize - 1, cell[1] as usize - 1);
                grid[r][c] = 0;
            }
            let mut q = std::collections::VecDeque::new();
            for i in 0..col {
                if grid[0][i as usize] > 0 {
                    q.push_back((0, i));
                    grid[0][i as usize] = 0;
                }
            }
            let mut found = false;
            while let Some((x, y)) = q.pop_front() {
                for d in 0..dirs.len() - 1 {
                    let (nx, ny) = (x + dirs[d], y + dirs[d + 1]);
                    if nx < 0
                        || nx >= row
                        || ny < 0
                        || ny >= col
                        || grid[nx as usize][ny as usize] == 0
                    {
                        continue;
                    }
                    if nx == row - 1 {
                        found = true;
                        break;
                    }
                    q.push_back((nx, ny));
                    grid[nx as usize][ny as usize] = 0;
                }
            }
            if found {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(row as usize,col as usize);
        let mn=m*n;
        let mut parent:Vec<usize>=(0..mn+2).collect();
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize> |{
            let (px,py)=(find(x,parent),find(y,parent));
            
            if px!=py{
                parent[py]=px;
            }
        };
        let mut valid=vec![vec![false;n];m];
        let mut ans=0;
        for (i,cell) in cells.iter().enumerate().rev(){
            let (x,y)=(cell[0] as usize-1,cell[1] as usize-1);
            valid[x][y]=true;
            for d in [0,1,0,-1,0].windows(2){
                let (nx,ny)=(cell[0]+d[0]-1,cell[1] +d[1]-1);
                if nx>=0 && nx<row && ny>=0 && ny<col && valid[nx as usize][ny as usize]{
                    unite(x*n+y,n*nx as usize+ny as usize,&mut parent);
                }
            }
            if x==0{
                unite(x*n+y,mn,&mut parent);
            }
            if x==m-1{
                unite(x*n+y,mn+1,&mut parent);
            }
            if find(mn,&mut parent)==find(mn+1,&mut parent){
                ans=i as i32;
                break
            }
        }
        ans
    }
}