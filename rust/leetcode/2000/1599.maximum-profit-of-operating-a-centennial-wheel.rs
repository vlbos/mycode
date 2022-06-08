/*
 * @lc app=leetcode id=1599 lang=rust
 *
 * [1599] Maximum Profit of Operating a Centennial Wheel
 */

// @lc code=start
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let (mut profit, mut maxp) = (0, 0);
        let mut ans = -1;
        let mut i = 0;
        let n = customers.len();
        let mut wait = 0;
        while wait > 0 || i < n {
            wait += if i < n { customers[i] } else { 0 };
            let up = wait.min(4);
            wait -= up;
            profit += up * boarding_cost - running_cost;
            if profit > maxp {
                maxp = profit;
                ans = i as i32 + 1;
            }
            i += 1;
        }
        if maxp > 0 {
            ans
        } else {
            -1
        }
    }
}
// @lc code=end
