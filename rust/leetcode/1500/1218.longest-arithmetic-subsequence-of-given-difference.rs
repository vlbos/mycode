/*
 * @lc app=leetcode id=1218 lang=rust
 *
 * [1218] Longest Arithmetic Subsequence of Given Difference
 */

// @lc code=start
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        for &v in &arr{
            m.insert(v,*m.get(&(v-difference)).unwrap_or(&0)+1);
            ans =ans.max(*m.get(&v).unwrap_or(&0));
        }
        ans
    }
}
// @lc code=end

