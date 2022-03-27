/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 * [860] 柠檬水找零
 */

// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change5 = 0;
        let mut change10 = 0;
        for b in &bills {
            if *b == 5 {
                change5 += 1;
            } else if *b == 10 {
                if change5 == 0 {
                    return false;
                }
                change5 -= 1;
                change10 += 1;
            } else {
                if change10 > 0 {
                    change10 -= 1;
                    if change5 > 0 {
                        change5 -= 1;
                    } else {
                        return false;
                    }
                } else {
                    if change5 > 2 {
                        change5 -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
