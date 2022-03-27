/*
 * @lc app=leetcode id=2162 lang=rust
 *
 * [2162] Minimum Cost to Set Cooking Time
 */

// @lc code=start
impl Solution {
    pub fn min_cost_set_time(
        start_at: i32,
        move_cost: i32,
        push_cost: i32,
        target_seconds: i32,
    ) -> i32 {
        let cost = |mm: i32, ss: i32| -> i32 {
            if mm < 0 || mm > 99 || ss < 0 || ss > 99 {
                return i32::MAX;
            }
            let digits = vec![mm / 10, mm % 10, ss / 10, ss % 10];
            let start = digits.iter().position(|x| *x > 0).unwrap();
            let mut prev = start_at;
            let mut ans = 0;
            for &d in digits.iter().skip(start) {
                if d != prev {
                    prev = d;
                    ans += move_cost;
                }
                ans += push_cost;
            }
            ans
        };
        let (mm, ss) = (target_seconds / 60, target_seconds % 60);
        cost(mm, ss).min(cost(mm - 1, ss + 60))
    }
}
// @lc code=end
