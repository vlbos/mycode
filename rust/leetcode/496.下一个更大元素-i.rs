/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::<i32>::new();
        let mut r = Vec::<i32>::new();
        let mut m = std::collections::HashMap::<i32, i32>::new();
        for n2 in &nums2 {
            while !s.is_empty() && s[s.len() - 1] < *n2 {
                m.insert(s[s.len() - 1], *n2);
                s.pop();
            }
            s.push(*n2);
        }
        for n1 in &nums1 {
            if let Some(n) = m.get(n1) {
                r.push(*n);
            } else {
                r.push(-1);
            }
        }
        r
    }
}
// @lc code=end
