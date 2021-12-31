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

