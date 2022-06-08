/*
 * @lc app=leetcode id=1328 lang=rust
 *
 * [1328] Break a Palindrome
 */

// @lc code=start
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let n = palindrome.len();
        if n==1{
            return String::new();
        }
        for (i, c) in palindrome.chars().enumerate() {
            if i == n / 2 {
                break;
            }
            if c != 'a' {
                let mut ans = palindrome.chars().collect::<Vec<char>>();
                ans[i] = 'a';
                return ans.iter().collect();
            }
        }
        let mut ans = palindrome.chars().collect::<Vec<char>>();
        ans[n - 1] = 'b';
        ans.iter().collect()
    }
}
// @lc code=end
