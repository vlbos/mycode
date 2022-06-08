/*
 * @lc app=leetcode id=870 lang=rust
 *
 * [870] Advantage Shuffle
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut n1 = nums1.clone();
        let mut n2 = nums2.clone();
        n1.sort();
        n2.sort();
        let mut a = HashMap::new();
        let mut r = VecDeque::new();
        let mut j = 0;
        for n in &n1 {
            if *n > n2[j] {
                a.entry(n2[j]).or_insert(VecDeque::new()).push_back(*n);
                j += 1;
            } else {
                r.push_back(*n);
            }
        }
        let mut ans = vec![0; nums2.len()];
        for (i,v) in nums2.iter().enumerate() {
            if let Some(mut n) = a.get_mut(v) {
                if !n.is_empty() {
                    ans[i] = n.pop_front().unwrap_or(0);
                } else {
                    ans[i] = r.pop_front().unwrap_or(0);
                }
            } else {
                ans[i] = r.pop_front().unwrap_or(0);
            }
        }
        ans
    }
}
// @lc code=end
