/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let (mut l, mut r) = (vec![0; n], vec![n as i32; n]);
        let mut s = Vec::new();
        for i in 0..n {
            while !s.is_empty() && heights[s[s.len() - 1]] >= heights[i] {
                r[s[s.len() - 1]] = i as i32;
                s.pop();
            }
            l[i] = if s.is_empty() {
                -1
            } else {
                s[s.len() - 1] as i32
            };
            s.push(i);
        }
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max((r[i] - l[i] - 1) * heights[i]);
        }
        ans
    }
}
// @lc code=end
