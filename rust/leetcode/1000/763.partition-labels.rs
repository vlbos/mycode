/*
 * @lc app=leetcode id=763 lang=rust
 *
 * [763] Partition Labels
 */

// @lc code=start
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            *m.entry(c).or_insert(i) = i;
        }
        let mut start = 0;
        let mut end = 0;
        let mut ans = Vec::new();

        for (i, c) in s.chars().enumerate() {
                end = end.max(*m.get(&c).unwrap());
                if i == end {
                    ans.push((end - start + 1) as i32);
                    start = end + 1;
                }
        }
        ans
    }
}
// @lc code=end
