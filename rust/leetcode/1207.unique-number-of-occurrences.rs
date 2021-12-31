/*
 * @lc app=leetcode id=1207 lang=rust
 *
 * [1207] Unique Number of Occurrences
 */

// @lc code=start
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut m = std::collections::HashMap::new();
    let mut s = std::collections::HashSet::new();
    for a in &arr{
            let mut n = m.entry(*a).or_insert(0);
            *n+=1;
    }
    for (k,v) in m{
            if s.contains(&v){
                return false;
            }
            s.insert(v);
    }
    true
    }
}
// @lc code=end

