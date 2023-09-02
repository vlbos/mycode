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

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        fn find(x:i32,parent:&mut HashMap<i32,i32>,rank:&mut HashMap<i32,i32>)->i32{
            if let Some(&px)=parent.get(&x){
                 if px!=x{
                     let pa=find(px,parent,rank);
                     parent.insert(x,pa);
                 }
            }else{
                parent.insert(x,x);
                rank.insert(x,1);
            }
            parent[&x]
        }
        let unite=|x:i32,y:i32,parent:&mut HashMap<i32,i32>,rank:&mut HashMap<i32,i32>|{
            let (px,py)=(find(x,parent,rank),find(y,parent,rank));
            if px!=py{
                let (px,py)=if rank[&px]<rank[&py]{
                    (py,px)
                }else{(px,py)};
                let ranky=rank[&py];
                rank.entry(px).and_modify(|mut r| *r+=ranky);
                parent.insert(py,px);
            }
        };
        let mut parent=HashMap::new();
        let mut rank=HashMap::new();
        for s in &stones{
            unite(s[0],s[1]+10001,&mut parent,&mut rank);
        }
        stones.len() as i32-parent.iter().filter(|(&k,&v)| k==v).count() as i32
    }
}