/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

// @lc code=start
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.len()<2 || num_courses < 2 {
            return true;
        }
        let mut lefts = prerequisites
            .iter()
            .map(|x| x[1])
            .collect::<std::collections::HashSet<_>>();
        let mut rights = prerequisites
            .iter()
            .map(|x| x[0])
            .collect::<std::collections::HashSet<_>>();
        let mut ps = std::collections::HashMap::new();
        let mut pps = std::collections::HashMap::new();
        prerequisites.iter().fold(&mut ps, |mut ps, p| {
            let mut e = ps.entry(p[1]).or_insert(vec![]);
            e.push(p[0]);
            ps
        });
        prerequisites.iter().fold(&mut pps, |mut ps, p| {
            let mut e = ps.entry(p[0]).or_insert(std::collections::HashSet::new());
            e.insert(p[1]);
            ps
        });
        let count = lefts.union(&rights).count();
        let mut ans = lefts
            .difference(&rights)
            .collect::<std::collections::HashSet<_>>();
        let mut presize= ans.len();
        loop {
            presize =ans.len();
            for (k, v) in &ps {
                if ans.contains(k) {
                    for n in v{
                        if let Some(vv)=pps.get_mut(n){
                              vv.remove(k);
                              if vv.is_empty(){
                                 ans.insert(n);
                                 pps.remove(n);
                              }
                        }
                    }
                    
                    if ans.len() == count{
                        return true;
                    }
                    
                }
            }
            if ans.len() == presize {
                break;
            }
        }

        false
    }
}
// @lc code=end
