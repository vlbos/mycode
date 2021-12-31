/*
 * @lc app=leetcode id=1217 lang=rust
 *
 * [1217] Minimum Cost to Move Chips to The Same Position
 */

// @lc code=start
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut e = 0;
        for p in &position{
            if *p%2==0{
            e+=1;
            }
        }
        (position.len() as i32-e).min(e)
    }
}
// @lc code=end

