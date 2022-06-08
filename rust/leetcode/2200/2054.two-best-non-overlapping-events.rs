/*
 * @lc app=leetcode id=2054 lang=rust
 *
 * [2054] Two Best Non-Overlapping Events
 */

// @lc code=start
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut sorted_events = Vec::new();
        for e in &events {
            sorted_events.push(vec![e[0], 0, e[2]]);
            sorted_events.push(vec![e[1], 1, e[2]]);
        }
        sorted_events.sort_by_key(|x| vec![x[0],x[1]]);
        let mut ans = 0;
        let mut best_first = 0;
        for e in &sorted_events {
            if e[1] == 0 {
                ans = ans.max(e[2] + best_first);
            } else {
                best_first = best_first.max(e[2]);
            }
        }
        ans
    }
}
// @lc code=end
