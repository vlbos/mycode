/*
 * @lc app=leetcode id=2032 lang=rust
 *
 * [2032] Two Out of Three
 */

// @lc code=start
impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut mask = std::collections::HashMap::new();
        let nn=vec![nums1, nums2, nums3];
        for (i, nums) in nn.iter().enumerate() {
            for num in nums {
                *mask.entry(num).or_insert(0) |= 1 << i;
            }
        }
        let mut ans = Vec::new();
        for (&k, &v) in &mask {
            if v > 2 && v != 4 {
                ans.push(*k);
            }
        }
        ans
    }
}
// @lc code=end
