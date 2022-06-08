/*
 * @lc app=leetcode id=1466 lang=rust
 *
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 */

// @lc code=start
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
         let mut s = std::collections::HashSet::new();
        let mut ans = 0;
        use std::collections::HashMap;
        let (mut indeg,mut outdeg)= (HashMap::new(),HashMap::new());
        for c in &connections{
        outdeg.entry(c[0]).or_insert(Vec::new()).push(c[1]);
        indeg.entry(c[1]).or_insert(Vec::new()).push(c[0]);
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        s.insert(0);
        while let Some(c)=q.pop_front(){
            if let Some(ocv)=outdeg.get(&c){
                for &oc in ocv{
                if !s.contains(&oc){
                    q.push_back(oc);
                    s.insert(oc);
                    ans+=1;
                }
                 }
            }
            if let Some(icv)=indeg.get(&c){
                for &ic in icv{
                    if !s.contains(&ic){
                        q.push_back(ic);
                        s.insert(ic);
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end

