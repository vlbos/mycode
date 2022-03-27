/*
 * @lc app=leetcode id=1904 lang=rust
 *
 * [1904] The Number of Full Rounds You Have Played
 */

// @lc code=start
impl Solution {
    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        let mut li: Vec<i32> = login_time
            .split(':')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut lo: Vec<i32> = logout_time
            .split(':')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let lit = li[0] * 60 + li[1];
        let mut lot = lo[0] * 60 + lo[1];
        if lit > lot {
            lot+=1440;
        }
        lot=lot/ 15*15;
        0i32.max((lot-lit)/15)
    }
}
// @lc code=end
