/*
 * @lc app=leetcode id=1441 lang=rust
 *
 * [1441] Build an Array With Stack Operations
 */

// @lc code=start
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut j = 1;
        for t in &target {
            while j <= n && j < *t {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
                j += 1;
            }
            if j == *t {
                ans.push("Push".to_string());
                j+=1;
            }
        }
        ans
    }
}
// @lc code=end
