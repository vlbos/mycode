/*
 * @lc app=leetcode.cn id=905 lang=rust
 *
 * [905] 按奇偶排序数组
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums;
        let mut i = 0;
        let mut j = n.len() - 1;
        while i < j {
            while i < j && n[i] % 2 == 0 {
                i += 1;
            }
            if i >= j {
                return n;
            }
            while i < j && n[j] % 2 != 0 {
                j -= 1;
            }
            if i >= j {
                return n;
            }
            let t = n[i];
            n[i] = n[j];
            n[j] = t;
            i += 1;
            j -= 1;
        }
        n
    }
}
// @lc code=end
