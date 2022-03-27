/*
 * @lc app=leetcode id=1774 lang=rust
 *
 * [1774] Closest Dessert Cost
 */

// @lc code=start
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut t:Vec<(i32,i32)> = base_costs.iter().map(|x| ((*x - target).abs(), *x)).collect();
        for &c in &topping_costs {
            for (_,b) in t.clone()  {
                t.push((((b + c) - target).abs(), b + c));
                t.push((((b + 2 * c) - target).abs(), b + 2 * c));
            }
        }
        t.sort();
        t[0].1
    }
}
// @lc code=end
