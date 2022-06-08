/*
 * @lc app=leetcode id=1346 lang=rust
 *
 * [1346] Check If N and Its Double Exist
 */

// @lc code=start
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut s = std::collections::HashSet::new();
        for a in &arr {
            if s.get(a).is_some() {
                return true;
            } else if *a % 2 == 0 {
                s.insert(*a / 2);
            }
            s.insert(*a * 2);
        }
        false
    }
}
// @lc code=end
