/*
 * @lc app=leetcode id=1905 lang=rust
 *
 * [1905] Count Sub Islands
 */

// @lc code=start
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
         let (m, n) = (grid2.len(), grid2[0].len());
        let mut visited = vec![vec![false; n]; m];
        fn dfs(
            grid1: &Vec<Vec<i32>>,
            grid2: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            i: usize,
            j: usize,
        ) -> bool {
            let (m, n) = (grid2.len() as i32, grid2[0].len() as i32);
            visited[i][j] = true;
            let dirs = [0, 1, 0, -1, 0];
            let mut ans = true;
            for k in 0..dirs.len() - 1 {
                let (x, y) = (i as i32 + dirs[k], j as i32 + dirs[k + 1]);
                if x < 0 || x >= m || y < 0 || y >= n {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if grid2[x][y] == 0 || visited[x][y] {
                    continue;
                }
                if grid1[x][y] != grid2[x][y] {
                    ans = false;
                }
                if !dfs(grid1, grid2, visited, x, y){
                    ans = false;
                }
            }
            ans
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 0 || grid1[i][j] != grid2[i][j] || visited[i][j] {
                    continue;
                }
                if dfs(&grid1, &grid2, &mut visited, i, j) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let (m,n)=(grid1.len(),grid1[0].len());
        let mut parent:Vec<usize>=(0..m*n).collect();
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
        for (i,row) in grid2.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==0{
                    continue
                }
                if i>0 && grid2[i-1][j]==1{
                    unite(i*n+j,(i-1)*n+j,&mut parent);
                }
                if j>0 && grid2[i][j-1]==1{
                    unite(i*n+j,i*n+j-1,&mut parent);
                }
            }
        }
        let mut candidate=std::collections::HashMap::new();
        for (i,row) in grid2.iter().enumerate(){
            for (j,&v) in row.iter().enumerate(){
                if v==0{
                    continue
                }
                let c=find(i*n+j,&mut parent);
                if !*candidate.get(&c).unwrap_or(&true){
                    continue
                }
                if  grid1[i][j]==0{
                    candidate.insert(c,false);
                    continue
                }
                candidate.entry(c).or_insert(true);
            }
        }
        candidate.values().filter(|&x|*x).count() as _
    }
}