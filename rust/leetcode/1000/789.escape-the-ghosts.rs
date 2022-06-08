/*
 * @lc app=leetcode id=789 lang=rust
 *
 * [789] Escape The Ghosts
 */

// @lc code=start
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let md =
            |p1: &Vec<i32>, p2: &Vec<i32>| -> i32 { (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs() };
        let d = md(&vec![0, 0], &target);
        for g in &ghosts {
            let gd = md(g, &target);
            if gd <= d {
                return false;
            }
        }
        true
    }
}
// @lc code=end
