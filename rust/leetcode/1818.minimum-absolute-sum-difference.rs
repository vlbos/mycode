/*
 * @lc app=leetcode id=1818 lang=rust
 *
 * [1818] Minimum Absolute Sum Difference
 */

// @lc code=start
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1 == nums2 {
            return 0;
        }
        let p = 1000_000_007;
        let mut rec = nums1.clone();
        rec.sort();
        let mut sum = 0;
        let mut maxn = 0;
        for (i, &v) in nums1.iter().enumerate() {
            let d = (v - nums2[i]).abs();
            sum += d as i64;
            sum %= p;
            let j = rec.binary_search(&nums2[i]).unwrap_or_else(|x|x);
            if j>0 {
                maxn=maxn.max( d - (nums2[i] - rec[j-1]));
            }   
            if j<nums1.len() {
                maxn=maxn.max( d - (rec[j] - nums2[i]));
            }   
        }
        ((sum - maxn as i64+p) % p) as _
    }
}
// @lc code=end
