/*
 * @lc app=leetcode id=1377 lang=rust
 *
 * [1377] Frog Position After T Seconds
 */

// @lc code=start
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        use std::collections::HashSet;
        let mut g = std::collections::HashMap::new();
        for e in &edges {
            g.entry(e[0]).or_insert(HashSet::new()).insert(e[1]);
            g.entry(e[1]).or_insert(HashSet::new()).insert(e[0]);
        }
        let mut visited = HashSet::from([1]);
        let mut q = std::collections::VecDeque::from([(1, 1.0, 0)]);
        while let Some((id, p, ut)) = q.pop_front() {
            if ut == t && id == target {
                return p;
            }
            if ut >= t {
                continue;
            }
            let cnt = g
                .get(&id)
                .unwrap_or(&HashSet::new())
                .iter()
                .filter(|x| !visited.contains(x))
                .count() as f64;
            let mut not_find = true;
            for v in g.get(&id).unwrap_or(&HashSet::new()) {
                if visited.contains(v) {
                    continue;
                }
                not_find = false;
                visited.insert(*v);
                q.push_back((*v, p / cnt, ut + 1));
            }
            if not_find {
                q.push_back((id, p, ut + 1));
            }
        }
        0.0
    }
}
// @lc code=end
