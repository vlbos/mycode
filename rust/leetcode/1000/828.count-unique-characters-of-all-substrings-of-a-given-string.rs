/*
 * @lc app=leetcode id=828 lang=rust
 *
 * [828] Count Unique Characters of All Substrings of a Given String
 */

// @lc code=start
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
       let mut index = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            index.entry(c).or_insert(Vec::new()).push(i);
        }
        let mut ans = 0i64;
        for a in index.values() {
            for (i, &v) in a.iter().enumerate() {
                let v = v as i64;
                let prev = if i == 0 { -1 } else { a[i - 1] as i64};
                let next = if i == a.len() - 1 {
                    s.len() 
                } else {
                    a[i + 1] 
                } as i64;
                ans += (v - prev) * (next - v);
            }
        }
        ans as _
    }
}
// @lc code=end
