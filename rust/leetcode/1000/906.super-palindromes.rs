/*
 * @lc app=leetcode id=906 lang=rust
 *
 * [906] Super Palindromes
 */

// @lc code=start
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let (l, r) = (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap());
        let magic = 100_000;
        let reverse = |mut x: i64| -> i64 {
            let mut ans = 0;
            while x > 0 {
                ans = ans * 10 + x % 10;
                x /= 10;
            }
            ans
        };
        let is_palindrome = |x: i64| -> bool { x == reverse(x) };
        let mut ans = 0;
        for k in 1..magic {
            let mut s = k.to_string();
            let t = s.clone() + s[..s.len() - 1].chars().rev().collect::<String>().as_str();
            let v = t.parse::<i64>().unwrap().pow(2);
            if v > r {
                break;
            }
            if v >= l && is_palindrome(v) {
                ans += 1;
            }
        }
        for k in 1..magic {
            let mut s = k.to_string();
            let t = s.clone() + s.chars().rev().collect::<String>().as_str();
            let v = t.parse::<i64>().unwrap().pow(2);
            if v > r {
                break;
            }
            if v >= l && is_palindrome(v) {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
