/*
 * @lc app=leetcode id=1648 lang=rust
 *
 * [1648] Sell Diminishing-Valued Colored Balls
 */

// @lc code=start
impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut inventory = inventory;
        inventory.sort_by(|x, y| y.cmp(&x));
        let mut ans = 0;
        let mut orders = orders as i64;
        for (i, &v) in inventory.iter().enumerate() {
            let v = v as i64;
            let mut x = v;
            if i < inventory.len() - 1 {
                x -= inventory[i + 1] as i64;
            }
            let n = i as i64 + 1;
            let t = orders.min(n * x);
            let r = t / n;
            let c = t % n;
            ans += (n * r * v - n * (r * (r - 1) / 2)) % 1000_000_007;
            ans += ((v - r) * c) % 1000_000_007;
            ans %= 1000_000_007;
            orders -= t;
            if orders == 0 {
                break;
            }
        }
        ans as _
    }
}
// @lc code=end
