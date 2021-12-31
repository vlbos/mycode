/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 */

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut r = Vec::<i32>::new();
        for n in &nums {
            if r.contains(n) {
                continue;
            }
            if r.is_empty() {
                r.push(*n);
                continue;
            }
            r.push(*n);
            r.sort();
            if r.len()>3{
            r.remove(0);
            }
        }
        if r.len() < 3 {
            r[r.len()-1]
        } else {
            r[0]
        }
    }
}
// @lc code=end

