/*
 * @lc app=leetcode id=1545 lang=rust
 *
 * [1545] Find Kth Bit in Nth Binary String
 */

// @lc code=start
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if k == 1 {
            return '0';
        }
        let mid = 1 << (n - 1);
        if k == mid {
            return '1';
        }
        if k < mid {
            return Self::find_kth_bit(n - 1, k);;
        }

        if Self::find_kth_bit(n - 1, mid*2-k) == '0' {
            '1'
        } else {
            '0'
        }
    }
}
// @lc code=end
