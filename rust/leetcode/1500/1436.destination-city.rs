/*
 * @lc app=leetcode id=1436 lang=rust
 *
 * [1436] Destination City
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut r = paths.iter().cloned().map(|x|x[1].clone()).collect::<HashSet<String>>();
        let mut l =  paths.iter().cloned().map(|x|x[0].clone()).collect::<HashSet<String>>();
        (*(r.difference(&l).into_iter().collect::<Vec<&String>>()[0])).clone()
    }
}
// @lc code=end

