/*
 * @lc app=leetcode id=1879 lang=rust
 *
 * [1879] Minimum XOR Sum of Two Arrays
 */

// @lc code=start
impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let n1 = 1 << n;
        let mut f = vec![i32::MAX; n1];
        f[0] = 0;
        for mask in 1..n1 {
            for i in 0..n {
                if mask & (1 << i) > 0 {
                    f[mask] =
                        f[mask].min(f[mask ^ (1 << i)] + (nums1[mask.count_ones() as usize - 1] ^ nums2[i]));
                }
            }
        }
        f[n1 - 1]
    }
}
// @lc code=end
