/*
 * @lc app=leetcode id=886 lang=rust
 *
 * [886] Possible Bipartition
 */

// @lc code=start
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n+1];
        for p in &dislikes {
            graph[p[0] as usize].push(p[1] as usize);
            graph[p[1] as usize].push(p[0] as usize);
        }
        let mut color = std::collections::HashMap::new();
        fn dfs(
            graph: &Vec<Vec<usize>>,
            color: &mut std::collections::HashMap<usize, bool>,
            i: usize,
            c: bool,
        )->bool {
            if color.contains_key(&i) {
                return *color.get(&i).unwrap_or(&false) == c;
            }
            color.insert(i, c);
            for node in &graph[i] {
                if !dfs(graph, color, *node, !c) {
                    return false;
                }
            }
            true
        }
        for node in 0..graph.len() {
            if !color.contains_key(&node) && !dfs(&graph, &mut color, node, true) {
                return false;
            }
        }
        true
    }
}
// @lc code=end


impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n=n as usize;
        let mut parent:Vec<usize>=(0..n).collect();
        let mut rank=vec![1;n];
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
             let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>,rank:&mut Vec<usize>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
               let (px,py)= if rank[px]<rank[py]{
                    (py,px)
                }else{(px,py)};
                rank[px]+=rank[py];
                parent[py]=px;
            }
        };
        let mut g=vec![vec![];n];
        for dislike in &dislikes{
            let (x,y)=(dislike[0] as usize-1,dislike[1] as usize-1);
            g[x].push(y);
            g[y].push(x);
        }
        for (x,nodes) in g.iter().enumerate(){
            for &y in nodes{
                    unite(nodes[0],y,&mut parent,&mut rank);
                    if find(x,&mut parent)==find(y,&mut parent){
                        return false
                    }
            }
        }
        true
    }
}