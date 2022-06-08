/*
 * @lc app=leetcode id=210 lang=rust
 *
 * [210] Course Schedule II
 */

// @lc code=start
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        if prerequisites.is_empty() {
            return (0..num_courses).collect();
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
        let mut ans = lefts.difference(&rights).map(|x| *x).collect::<Vec<_>>();
        let mut presize = ans.len();
        loop {
            presize = ans.len();
            for (k, v) in &ps {
                if ans.contains(&k) {
                    for n in v {
                        if let Some(vv) = pps.get_mut(n) {
                            vv.remove(k);
                            if vv.is_empty() {
                                ans.push(*n);
                                pps.remove(n);
                            }
                        }
                    }

                    if ans.len() == count {
                        if ans.len() < num_courses as usize {
                            for i in 0..num_courses {
                                if !ans.contains(&i) {
                                    ans.insert(0, i);
                                }
                            }
                        }
                        return ans;
                    }
                }
            }
            if ans.len() == presize {
                break;
            }
        }
        
        Vec::new()
    }
}
// @lc code=end
