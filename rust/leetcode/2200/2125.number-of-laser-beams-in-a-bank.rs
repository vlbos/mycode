/*
 * @lc app=leetcode id=2125 lang=rust
 *
 * [2125] Number of Laser Beams in a Bank
 */

// @lc code=start
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut pre = 0;
        for s in &bank {
            let c = s.chars().filter(|x| *x == '1').count();
            if pre > 0 {
                ans += pre * c;
            }
            if c>0{
                pre = c;
            }
        }
        ans as _
    }
}
// @lc code=end
