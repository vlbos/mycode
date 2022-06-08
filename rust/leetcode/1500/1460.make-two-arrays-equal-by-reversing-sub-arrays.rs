/*
 * @lc app=leetcode id=1460 lang=rust
 *
 * [1460] Make Two Arrays Equal by Reversing Sub-arrays
 */

// @lc code=start
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        if target==arr{
        return true;
        }
        let mut t = target;
        let mut a = arr;
        t.sort();
        a.sort();
        t==a
    }
}
// @lc code=end

