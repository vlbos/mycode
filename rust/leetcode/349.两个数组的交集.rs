/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

// @lc code=start
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut r = Vec::<i32>::new();
        for n in nums1 {
            if nums2.contains(&n) && !r.contains(&n) {
                r.push(n)
            }
        }
        r
    }
}
// @lc code=end
