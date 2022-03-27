/*
 * @lc app=leetcode id=1029 lang=rust
 *
 * [1029] Two City Scheduling
 */

// @lc code=start
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_by_key(|x|x[0]-x[1]);
        let n = costs.len()/2;
        let a = costs[..n].iter().map(|x|x[0]).sum::<i32>();
        let b = costs[n..].iter().map(|x|x[1]).sum::<i32>();
        a+b
    }
}
// @lc code=end

