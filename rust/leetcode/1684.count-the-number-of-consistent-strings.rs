/*
 * @lc app=leetcode id=1684 lang=rust
 *
 * [1684] Count the Number of Consistent Strings
 */

// @lc code=start
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut a = vec![0; 26];
        for c in allowed.chars() {
            a[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let mut ans = 0;
        let mut flag = true;
        for w in &words {
            flag = true;
            for c in w.chars() {
                let i = (c as u8 - 'a' as u8) as usize;
                if a[i] == 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
