/*
 * @lc app=leetcode id=1694 lang=rust
 *
 * [1694] Reformat Phone Number
 */

// @lc code=start
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut n = number;
        n = n.replace(" ", "");
        n = n.replace("-", "");
        if n.len() < 3 {
            return n;
        }
        // println!("{}", n);
        let mut ans = String::new();
        let d = if n.len() % 3 == 1 { 1 } else { 0 };
        for i in 0..n.len() / 3 - d {
            if !ans.is_empty() {
                ans += "-";
            }
            // println!("{}", &n[i*3..(i+1)*3]);
            ans += &n[i * 3..(i + 1) * 3];
        }
        if n.len() % 3 == 1 {
            if n.len() > 4 {
                ans += "-";
            }
            ans += &n[n.len() - 4..n.len() - 2];
            ans += "-";
            ans += &n[n.len() - 2..];
        } else if n.len() % 3 == 2 {
            ans += "-";
            ans += &n[n.len() - 2..];
        }
        ans
    }
}
// @lc code=end
