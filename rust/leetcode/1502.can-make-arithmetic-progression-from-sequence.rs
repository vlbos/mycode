/*
 * @lc app=leetcode id=1502 lang=rust
 *
 * [1502] Can Make Arithmetic Progression From Sequence
 */

// @lc code=start
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        if arr.len() == 2 {
         return true;
        }
        let mut a = arr;
        a.sort();
        let d = a[0] - a[1];
        for i in 1..a.len() - 1 {
            if a[i] - a[i + 1] != d {
                return false;
            }
        }
        true
    }
}
// @lc code=end
