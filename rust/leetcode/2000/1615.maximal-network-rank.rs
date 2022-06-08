/*
 * @lc app=leetcode id=1615 lang=rust
 *
 * [1615] Maximal Network Rank
 */

// @lc code=start
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
       use std::collections::HashSet;
        let mut m = std::collections::HashMap::new();
        let mut ans = 0;
        for r in &roads {
            m.entry(r[0]).or_insert(HashSet::new()).insert(r[1]);
            m.entry(r[1]).or_insert(HashSet::new()).insert(r[0]);
        }
        for i in 0..n {
            for j in i + 1..n {
                let mut rank = m.get(&i).unwrap_or(&HashSet::new()).len() + m.get(&j).unwrap_or(&HashSet::new()).len();
                if m.get(&i).unwrap_or(&HashSet::new()).contains(&j) {
                    rank -= 1;
                }
                ans = ans.max(rank);
            }
        }

        ans as _
    }
}
// @lc code=end
