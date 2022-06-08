/*
 * @lc app=leetcode id=946 lang=rust
 *
 * [946] Validate Stack Sequences
 */

// @lc code=start
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let n = pushed.len();
        let mut s = Vec::new();
        let mut j = 0;
        for &v in &pushed{
            s.push(v);
            while !s.is_empty() && j<n && *s.last().unwrap()==popped[j]{
                s.pop();
                j+=1;
            }
        }
        j==n
    }
}
// @lc code=end

