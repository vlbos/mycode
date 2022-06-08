/*
 * @lc app=leetcode id=1817 lang=rust
 *
 * [1817] Finding the Users Active Minutes
 */

// @lc code=start
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut m = HashMap::new();
        for l in &logs {
            m.entry(l[0]).or_insert(HashSet::new()).insert(l[1]);
        }
        let mut ans = vec![0; k as usize];
        for (_, v) in &m {
            ans[v.len()-1] += 1;
        }
        ans
    }
}
// @lc code=end
