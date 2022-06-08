/*
 * @lc app=leetcode id=1081 lang=rust
 *
 * [1081] Smallest Subsequence of Distinct Characters
 */

// @lc code=start
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut vis = vec![false; 26];
        let mut num = vec![0; 26];
        for b in s.bytes() {
            num[(b - b'a') as usize] += 1;
        }
        let mut stack = Vec::new();
        for b in s.bytes() {
            if vis[(b - b'a') as usize] {
                num[(b - b'a') as usize] -= 1;
                continue;
            }
            while !stack.is_empty() && *stack.last().unwrap() > b {
                let i = (*stack.last().unwrap() - b'a') as usize;
                if num[i] > 0 {
                    vis[i] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            vis[(b - b'a') as usize] = true;
            stack.push(b);
            num[(b - b'a') as usize] -= 1;
        }
        String::from_utf8(stack).unwrap()
    }
}
// @lc code=end
