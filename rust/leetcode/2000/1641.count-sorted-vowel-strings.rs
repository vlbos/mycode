/*
 * @lc app=leetcode id=1641 lang=rust
 *
 * [1641] Count Sorted Vowel Strings
 */

// @lc code=start
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut a = vec![1; 5];
        for _ in 2..=n {
            for j in (0..=3).rev() {
                a[j] += a[j + 1];
            }
        }
        a.iter().sum::<i32>()
    }
}
// @lc code=end
