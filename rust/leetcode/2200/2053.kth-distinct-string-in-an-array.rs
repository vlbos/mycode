/*
 * @lc app=leetcode id=2053 lang=rust
 *
 * [2053] Kth Distinct String in an Array
 */

// @lc code=start
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut m = std::collections::HashMap::new();
        for s in &arr {
            *m.entry(s.clone()).or_insert(0) += 1;
        }
        let mut cnt = 0;
        for s in &arr {
            if *m.get(s).unwrap() == 1 {
                cnt += 1;
                if cnt == k {
                    return s.clone();
                }
            }
        }
        String::new()
    }
}
// @lc code=end
