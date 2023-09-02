/*
 * @lc app=leetcode id=827 lang=rust
 *
 * [827] Making A Large Island
 */

// @lc code=start
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
       let n = grid.len();
        let mut grid = grid;
        let mut area = std::collections::HashMap::new();
        let mut index = 2;
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 1 {
                    area.insert(index, dfs(r, c,index, &mut grid));
                    index += 1;
                }
            }
        }
        fn dfs(r: usize, c: usize, index: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
            let n = grid.len();
            let mut ans = 1;
            grid[r][c] = index;
            for (nr, nc) in neighbors(r as i32, c as i32, n as i32) {
                if grid[nr][nc] == 1 {
                    ans += dfs(nr, nc, index, grid);
                }
            }
            ans
        }
        fn neighbors(r: i32, c: i32, n: i32)->Vec<(usize,usize)> {
            let dirs = [0, 1, 0, -1, 0];
            let mut ans = Vec::new();
            for i in 0..dirs.len() - 1 {
                let (nr, nc) = (r + dirs[i], c + dirs[i + 1]);
                if nr < 0 || nr >= n || nc < 0 || nc >= n {
                    continue;
                }
                ans.push((nr as usize, nc as usize));
            }
            ans
        }
        let mut ans = *area.values().max().unwrap_or(&0);
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 {
                    let mut seen = std::collections::HashSet::new();

                    for (nr, nc) in neighbors(r as i32, c as i32, n as i32) {
                        if grid[nr][nc] > 1 {
                            seen.insert(grid[nr][nc]);
                        }
                    }
                    let bns = seen.into_iter().map(|x| area[&x]).sum::<i32>() + 1;
                    ans = ans.max(bns);
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n=grid.len();
        let nn=n*n;
        let mut parent:Vec<usize>=(0..nn).collect();
        let mut size=vec![1;nn];
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>,size:&mut Vec<i32>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px]=py;
                size[py]+=size[px];
            }
        };
        for (i,row) in grid.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==0{
                    continue
                }
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni>=0 && ni<n as i32 && nj>=0 && nj<n as i32 && grid[ni as usize][nj as usize]==1{
                        unite(i*n+j,ni as usize*n+nj as usize,&mut parent,&mut size);
                    }
                }
            }
        }
        let mut ans=0;
        for (i,row) in grid.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==1{
                    ans=ans.max(size[find(i*n+j,&mut parent)]) ;
                    continue
                }
                let mut s=std::collections::HashSet::new();
                let mut sz=0;
                for d in [0,1,0,-1,0].windows(2){
                    let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
                    if ni>=0 && ni<n as i32 && nj>=0 && nj<n as i32 && grid[ni as usize][nj as usize]==1{
                        let r=find(ni as usize*n+nj as usize,&mut parent);
                        if !s.contains(&r){
                            sz+=size[r];
                            s.insert(r);
                        }
                    }
                }
                ans=ans.max(sz+1);
            }
        }
        ans
    }
}