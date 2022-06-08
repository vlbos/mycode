/*
 * @lc app=leetcode id=1713 lang=rust
 *
 * [1713] Minimum Operations to Make a Subsequence
 */

// @lc code=start
impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let n = target.len();
        let pos: std::collections::HashMap<i32, usize> = target
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        let mut d = Vec::new();
        for &v in &arr {
            if let Some(idx) = pos.get(&v) {
                match d.binary_search(&idx) {
                    Ok(i) | Err(i) if i < d.len() => d[i] = idx,
                    _ => d.push(idx),
                }
            }
        }
        (n - d.len()) as _
    }
}
// @lc code=end
