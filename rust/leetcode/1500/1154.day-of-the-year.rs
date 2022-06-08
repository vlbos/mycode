/*
 * @lc app=leetcode id=1154 lang=rust
 *
 * [1154] Day of the Year
 */

// @lc code=start
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let d = date
            .split("-")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut ans = 0;
        for i in 0..d[1] {
            if i == 0 {
                ans += d[2];
            } else if i == 2 {
                ans += 28;
                if d[0] % 4 == 0 && (d[0] % 100 != 0 ||  d[0] % 400 == 0) {
                    ans += 1;
                }
            } else if i == 4 || i == 6 || i == 9 || i == 11 {
                ans += 30;
            } else {
                ans += 31;
            }
        }
        ans
    }
}
// @lc code=end
