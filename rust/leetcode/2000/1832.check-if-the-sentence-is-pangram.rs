/*
 * @lc app=leetcode id=1832 lang=rust
 *
 * [1832] Check if the Sentence Is Pangram
 */

// @lc code=start
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut a = vec![0; 26];
        for c in sentence.chars() {
            a[(c as u8 - 'a' as u8) as usize] += 1;
        }
        a.iter().filter(|&x| *x == 0).count() == 0
    }
}
// @lc code=end
