/*
 * @lc app=leetcode id=547 lang=rust
 *
 * [547] Number of Provinces
 */

// @lc code=start
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut vis = vec![false;n];
        let mut ans = 0;
        for i in 0..n{
              if !vis[i]{
                vis[i]=true;
                dfs(&is_connected,&mut vis,i);
                 ans+=1;
              }
        }
        fn dfs(is_connected: &Vec<Vec<i32>>,vis: &mut Vec<bool>,j:usize){
              for i in 0..is_connected.len(){
              if is_connected[i][j]==1 && !vis[i]{
                     vis[i]=true;
                    dfs(&is_connected,vis,i);
              }
            }
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n=is_connected.len();
        let mut parent:Vec<usize>=(0..n).collect();
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
        for i in 0..n{
            for j in i+1..n{
                if is_connected[i][j]==1{
                    unite(i,j,&mut parent);
                }
            }
        }
        parent.into_iter().enumerate().filter(|(i,j)| i==j).count() as _
    }
}