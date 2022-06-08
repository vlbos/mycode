/*
 * @lc app=leetcode id=2139 lang=rust
 *
 * [2139] Minimum Moves to Reach Target Score
 */

// @lc code=start
impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
         if target == 1 {
            return 0;
        }
        let mut ans = 0;
        let mut t = target;
        for _ in 0..max_doubles {
            if t % 2 > 0 {
                t -= 1;
                ans += 1;
            }
            if t > 1 {
                t /= 2;
                ans += 1;
            }
            if t == 1 {
                break;
            }
        }
        if t  > 1 {
            ans += t-1;
        }
        ans
    }
}
// @lc code=end
