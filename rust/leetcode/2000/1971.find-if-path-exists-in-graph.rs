/*
 * @lc app=leetcode id=1971 lang=rust
 *
 * [1971] Find if Path Exists in Graph
 */

// @lc code=start
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        if n == 1 && edges.is_empty() && start == 0 && end == 0 {
            return true;
        }
        let mut s = std::collections::HashSet::new();
        for i in 0..2{
            for e in &edges {
                if e[0] == start || e[1] == start || s.contains(&e[0]) || s.contains(&e[1]) {
                    s.insert(e[0]);
                    s.insert(e[1]);
                }
            }
        }
        s.contains(&end)
    }
}
// @lc code=end
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut parent:Vec<i32>=(0..n).collect();
        fn find(x:i32,parent:&mut Vec<i32>)->i32{
            let px=parent[x as usize];
            if px!=x{
                parent[x as usize]=find(px,parent);
            }
             parent[x as usize]
        }
        let unite=|x:i32,y:i32,parent:&mut Vec<i32>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px as usize]=py;
            }
        };
        for e in &edges{
            unite(e[0],e[1],&mut parent);
        }
        find(source,&mut parent)==find(destination,&mut parent)
    }
}