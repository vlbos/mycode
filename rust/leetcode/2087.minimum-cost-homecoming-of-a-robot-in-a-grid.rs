/*
 * @lc app=leetcode id=2087 lang=rust
 *
 * [2087] Minimum Cost Homecoming of a Robot in a Grid
 */

// @lc code=start
impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
       let (r1, c1) = (start_pos[0] as usize, start_pos[1] as usize);
        let (r2, c2) = (home_pos[0] as usize, home_pos[1] as usize);
        let mut ans = 0;
        let (s, e) = if r2 >= r1 { (r1+1, r2+1) } else { (r2, r1) };
        ans += row_costs[s..e].iter().sum::<i32>();
        let (s, e) = if c2 >= c1 { (c1+1, c2+1) } else { (c2, c1) };
        ans += col_costs[s..e].iter().sum::<i32>();
        ans
    }
}
// @lc code=end
