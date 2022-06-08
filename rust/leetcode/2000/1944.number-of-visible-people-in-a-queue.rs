/*
 * @lc app=leetcode id=1944 lang=rust
 *
 * [1944] Number of Visible People in a Queue
 */

// @lc code=start
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0; n];
        let mut s = Vec::new();
        for i in (0..n).rev() {
            while !s.is_empty() {
                ans[i] += 1;
                if heights[i] > heights[s[s.len() - 1]] {
                    s.pop();
                } else {
                    break;
                }
            }
            s.push(i);
        }
        ans
    }
}
// @lc code=end
