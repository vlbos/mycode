/*
 * @lc app=leetcode id=947 lang=rust
 *
 * [947] Most Stones Removed with Same Row or Column
 */

// @lc code=start
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n= stones.len();
        let mut vis=  vec![false;n];
        let mut edges=  vec![vec![0;n];n];
        let mut m = std::collections::HashMap::new();
        for i in 0..n{
            m.entry(stones[i][0]).or_insert(Vec::new()).push(i);
            m.entry(stones[i][1]+10001).or_insert(Vec::new()).push(i);
        }
        for (_,v) in &m{
            for i in 1..v.len(){
                edges[v[i-1]].push(v[i]);
                edges[v[i]].push(v[i-1]);
            }
        }
        fn dfs(x:usize,edges:&Vec<Vec<usize>>,vis:&mut Vec<bool>){
            vis[x]=true;
            for &i in &edges[x]{
                if !vis[i]{
                    dfs(i,edges,vis);
                }
            }
        }
        let mut c = 0;
        for i in 0..n{
                 if !vis[i]{
                    c+=1;
                    dfs(i,&edges,&mut vis);
                }
        }
        (n-c) as _
    }
}
// @lc code=end

