/*
 * @lc app=leetcode id=983 lang=rust
 *
 * [983] Minimum Cost For Tickets
 */

// @lc code=start
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        fn dp(days: &Vec<i32>, costs: &Vec<i32>, memo: &mut Vec<i32>, i: usize) -> i32 {
            if i >= days.len() {
                return 0;
            }
            if memo[i] != -1 {
                return memo[i];
            }
            memo[i] = i32::MAX;
            let durations = [1, 7, 30];
            let mut j = i;
            for (c, d) in costs.iter().zip(&durations) {
                while j < days.len() && days[j] < days[i] + d {
                    j += 1;
                }
                memo[i] = memo[i].min(dp(days, costs, memo, j) + c);
            }
            memo[i]
        }
        let mut memo = vec![-1; days.len()];
        dp(&days, &costs, &mut memo, 0)
    }
}
// @lc code=end
