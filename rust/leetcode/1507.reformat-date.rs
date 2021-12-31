/*
 * @lc app=leetcode id=1507 lang=rust
 *
 * [1507] Reformat Date
 */

// @lc code=start
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let d = date.split_ascii_whitespace().collect::<Vec<&str>>();
        let day:i32 = d[0][..d[0].len()-2].parse().unwrap();
        let mv = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let month = mv.iter().position(|x|x==&d[1]).unwrap()+1;
        format!("{}-{:02}-{:02}",d[2],month,day)
    }
}
// @lc code=end

