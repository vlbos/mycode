/*
 * @lc app=leetcode id=1733 lang=rust
 *
 * [1733] Minimum Number of People to Teach
 */

// @lc code=start
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut p = HashSet::new();
        let mut m = HashMap::new();
        for f in &friendships {
            let (u, v) = (f[0] as usize - 1, f[1] as usize - 1);

            let lu = languages[u]
                .clone()
                .iter()
                .cloned()
                .collect::<HashSet<i32>>();
            let lv = languages[v]
                .clone()
                .iter()
                .cloned()
                .collect::<HashSet<i32>>();
            if lu.intersection(&lv).count() == 0 {
                p.insert(u);
                p.insert(v);
                for &l in &languages[u] {
                    m.entry(l).or_insert(HashSet::new()).insert(u);
                }
                for &l in &languages[v] {
                    m.entry(l).or_insert(HashSet::new()).insert(v);
                }
            }
        }
        if m.is_empty() {
            p.len() as _
        } else {
            (p.len() - m.values().map(|x| x.len()).max().unwrap()) as _
        }
    }
}
// @lc code=end
