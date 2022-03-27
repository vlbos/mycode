/*
 * @lc app=leetcode id=1953 lang=rust
 *
 * [1953] Maximum Number of Weeks for Which You Can Work
 */

// @lc code=start
impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let max = (*milestones.iter().max().unwrap()) as i64;
        let rest = milestones.iter().map(|x| (*x) as i64).sum::<i64>() - max;
        if max > rest + 1 {
            rest * 2 + 1
        } else {
            max + rest
        }
    }
}
// @lc code=end
