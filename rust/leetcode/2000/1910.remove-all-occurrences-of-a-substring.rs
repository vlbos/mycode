/*
 * @lc app=leetcode id=1910 lang=rust
 *
 * [1910] Remove All Occurrences of a Substring
 */

// @lc code=start
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut m = part.len();
        let mut pi1 = vec![0; m];
        let mut j = 0;
        let pb = part.as_bytes();
        for i in 1..m {
            while j > 0 && pb[i] != pb[j] {
                j = pi1[j - 1];
            }
            if pb[i] == pb[j] {
                j += 1;
            }
            pi1[i] = j;
        }
        let mut ans = String::new();
        let mut pi2 = vec![0];
        for c in s.chars() {
            ans.push(c);
            let mut j = *pi2.last().unwrap();
            let ch = c as u8;
            while j > 0 && ch != pb[j] {
                j = pi1[j - 1];
            }
            if ch == pb[j] {
                j += 1;
            }
            pi2.push(j);
            if j == m {
                pi2.truncate(pi2.len() - m);
                ans.truncate(ans.len() - m);
            }
        }
        ans
    }
}
// @lc code=end
