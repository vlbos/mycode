/*
 * @lc app=leetcode id=1743 lang=rust
 *
 * [1743] Restore the Array From Adjacent Pairs
 */

// @lc code=start
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for a in &adjacent_pairs {
            m.entry(a[0]).or_insert(Vec::new()).push(a[1]);
            m.entry(a[1]).or_insert(Vec::new()).push(a[0]);
        }
        let mut ans = vec![0; m.len()];
        for (&k, v) in &m {
            if v.len() == 1 {
                ans[0] = k;
                ans[1] = v[0];
                break;
            }
        }
        for i in 2..ans.len() {
            let adj = &m[&ans[i - 1]];
            ans[i] = if adj[0] == ans[i - 2] { adj[1] } else { adj[0] };
        }
        ans
    }
}
// @lc code=end
