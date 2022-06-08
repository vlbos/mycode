/*
 * @lc app=leetcode id=564 lang=rust
 *
 * [564] Find the Closest Palindrome
 */

// @lc code=start
impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let m = n.len() as u32;
        let mut candidates = vec![10i64.pow(m - 1) - 1, 10i64.pow(m) + 1];
        let self_prefix = n[..(m as usize + 1) / 2].parse::<i64>().unwrap();
        for x in self_prefix - 1..self_prefix + 2 {
            let mut x = x;
            let mut y = if m % 2 == 0 { x } else { x / 10 };
            while y > 0 {
                x = x * 10 + y % 10;
                y /= 10;
            }
            candidates.push(x);
        }
        let mut ans = -1;
        let self_number = n.parse::<i64>().unwrap();
        for &candidate in &candidates {
            if candidate != self_number {
                if ans == -1 {
                    ans = candidate;
                    continue;
                }
                let (cd, ad) = ((candidate - self_number).abs(), (ans - self_number).abs());
                if cd < ad ||( cd == ad && candidate < ans) {
                    ans = candidate;
                }
            }
        }
        ans.to_string()
    }
}
// @lc code=end
