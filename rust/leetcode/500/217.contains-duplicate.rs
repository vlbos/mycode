/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
         let mut v = Vec::<i32>::new();
        for i in nums {
            if v.contains(&i) {
                return true;
            } else {
                v.push(i);
            }
        }
        false
    }
}
// @lc code=end

