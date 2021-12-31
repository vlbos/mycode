/*
 * @lc app=leetcode id=310 lang=rust
 *
 * [310] Minimum Height Trees
 */

// @lc code=start
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n==1{
        return vec![0];}
        let mut m = std::collections::HashMap::new();
        let mut degree = vec![0;n as usize];
        for e in &edges {
            degree[e[0] as usize]+=1;
            degree[e[1] as usize]+=1;
            m.entry(e[0]).or_insert(vec![]).push(e[1]);
            m.entry(e[1]).or_insert(vec![]).push(e[0]);
        }
        let mut mht = i32::MAX;
        let mut ans = Vec::new();
        let mut q = std::collections::VecDeque::new();
        
        for i in 0..n {
            if degree[i as usize]==1{
                q.push_back(i);
            }
        }
        while !q.is_empty(){
                ans.clear();
                for _ in 0..q.len(){
                     let i = q.pop_front().unwrap();
                      ans.push(i);
                      if let Some(ns)=m.get(&i){
                            for j in ns{
                                degree[*j as usize]-=1;
                                if degree[*j as usize]==1{
                                    q.push_back(*j);
                                }
                            }
                      }
                }
        }
        ans
    }
}
// @lc code=end
