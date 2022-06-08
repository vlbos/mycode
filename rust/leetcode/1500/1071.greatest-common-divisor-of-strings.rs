/*
 * @lc app=leetcode id=1071 lang=rust
 *
 * [1071] Greatest Common Divisor of Strings
 */

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}",&str1, &str2) != format!("{}{}",&str2, &str1) {
            return String::new();
        }
        fn gcd(x: usize, y: usize) -> usize {
            let mut x = x;
            let mut y = y;
            while y != 0 {
                let t = y;
                y = x % y;
                x = t;
            }
            x
        }
        (&str1[..gcd(str1.len(), str2.len())]).to_string()
    }
}
// @lc code=end
