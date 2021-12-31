/*
 * @lc app=leetcode id=831 lang=rust
 *
 * [831] Masking Personal Information
 */

// @lc code=start
impl Solution {
    pub fn mask_pii(s: String) -> String {
        if let Some(i) = s.find('@') {
            let mut s = s;
            let mut domain = s.split_off(i);
            domain.make_ascii_lowercase();
            s.make_ascii_lowercase();
            return format!(
                "{}*****{}{}",
                s.chars().nth(0).unwrap(),
                s.chars().last().unwrap(),
                domain
            );
        } else {
            let mut s = s;
            s = s.replace(&['+', '-', '(', ')', ' '][..],"");
            let n = s.len();
            if s.len() > 10 {
                let cc = s.len() - 10;
                return format!("+{}-***-***-{}", "*".repeat(cc), s.split_off(n - 4));
            } else {
                return format!("***-***-{}", s.split_off(n - 4));
            }
        }
        String::new()
    }
}
// @lc code=end
