/*
 * @lc app=leetcode id=1481 lang=rust
 *
 * [1481] Least Number of Unique Integers after K Removals
 */

// @lc code=start
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
         let mut m = std::collections::HashMap::new();
        for &a in &arr {
            *m.entry(a).or_insert(0) += 1;
        }
        let mut v: Vec<i32> = m.iter().map(|(k, v)| *v).collect();
        v.sort();
        let mut ans = v.len() as i32;
        let mut k = k;
        for &p in &v {
            if p <= k {
                ans -= 1;
            }
            k -= p;
            if k <= 0 {
                break;
            }
        }
        ans
    }
}
// @lc code=end
