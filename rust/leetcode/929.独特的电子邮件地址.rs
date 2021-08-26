/*
 * @lc app=leetcode.cn id=929 lang=rust
 *
 * [929] 独特的电子邮件地址
 */

// @lc code=start
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut r = Vec::new();
        for s in &emails {
            let mut nn = (*s).split("@").collect::<Vec<&str>>();
            if let Some(s) = nn[0].find("+") {
                nn[0] = &(nn[0])[0..s];
            }
            let tt = nn[0].replace(".", "");
            nn[0] = &tt;
            let ss = nn[0].to_string() + "@" + nn[1];
            if !r.contains(&ss) {
                r.push(ss);
            }
        }
        r.len() as i32
    }
}
// @lc code=end
