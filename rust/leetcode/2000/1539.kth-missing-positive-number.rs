/*
 * @lc app=leetcode id=1539 lang=rust
 *
 * [1539] Kth Missing Positive Number
 */

// @lc code=start
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 1;
        let mut s = 0;
        for a in &arr {
            while i < *a {
                s += 1;
                if s == k {
                    return i;
                }
                i += 1;
            }
            i += 1;
        }
        while s < k {
            s += 1;
            if s == k {
                return i;
            }
            i += 1;
        }
        s
    }
}
// @lc code=end
