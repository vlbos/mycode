/*
 * @lc app=leetcode id=433 lang=rust
 *
 * [433] Minimum Genetic Mutation
 */

// @lc code=start
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
          let  choices=['A', 'C', 'G', 'T'];
          if !bank.contains(&end){
                return -1;
          }
          if start==end{
            return 0;
          }
          let mut h = bank.iter().collect::<std::collections::HashSet<_>>();
          let mut q =std::collections::VecDeque::new();
        
          q.push_back((start,0));
          while let Some(g)=q.pop_front(){
               if g.0==end{
                    return g.1;
                }
                for (i,v) in g.0.chars().enumerate(){
                    for  c  in &choices{
                        let  t = format!("{}{}{}",&g.0[..i],c,&g.0[i+1..]);
                         if h.contains(&t){
                            q.push_back((t.clone(),g.1+1));
                            h.remove(&t);
                         }
                    }
                }
          }
        -1
        }
}
// @lc code=end

