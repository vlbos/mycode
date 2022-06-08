/*
 * @lc app=leetcode id=964 lang=rust
 *
 * [964] Least Operators to Express Number
 */

// @lc code=start
impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        use std::collections::HashMap;
        let mut memo = HashMap::new();
        fn dp(i: i32, target: i32, x: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if let Some(&j) = memo.get(&(i, target)) {
                return j;
            }
            let mut ans = 0;
            let cost = |x: i32| {
                if x > 0 {
                    x
                } else {
                    2
                }
            };
            if target == 0 {
                ans = 0;
            } else if target == 1 {
                ans = cost(i);
            } else if i >= 39 {
                ans = target + 1;
            } else {
                let (t, r) = (target / x, target % x);
                ans = (r * cost(i) + dp(i + 1, t, x, memo))
                    .min((x - r) * cost(i) + dp(i + 1, t + 1, x, memo));
            }
            memo.insert((i, target), ans);
            ans
        }
        dp(0, target, x, &mut memo) - 1
    }
}
// @lc code=end
