// [4019. Merge Close Characters II](https://leetcode.com/problems/merge-close-characters-ii/)

// You are given a string `s` consisting of lowercase English letters and an integer `k`.
// Two equal characters `s[i]` and `s[j]`, where `0 \<= i \< j \< s.length`, are considered **close** if `j - i \<= k`. All indices refer to the **current** string.
// Repeatedly perform the following operation until no close pair remains:
// * Among all close pairs `(i, j)`, choose the pair with the smallest `i`. If multiple pairs have the same `i`, choose the one with the smallest `j`.
// * Merge the right character into the left character by removing `s[j]` from `s`. The character `s[i]` remains unchanged, and the remaining characters are reindexed.
// Return the resulting string after performing all possible merges.
// **Example 1:**
// **Input:** s = "abca", k = 3
// **Output:** "abc"
// **Explanation:**
// * The characters `'a'` at indices 0 and 3 are close because `3 - 0 = 3 \<= k`.
// * Remove the right `'a'`, resulting in `s = "abc"`.
// * No close pair remains, so no further merges are performed.
// **Example 2:**
// **Input:** s = "aabca", k = 2
// **Output:** "abca"
// **Explanation:**
// * The characters `'a'` at indices 0 and 1 are close because `1 - 0 = 1 \<= k`.
// * Remove the right `'a'`, resulting in `s = "abca"`.
// * The remaining `'a'` characters are at indices 0 and 3. Since `3 - 0 = 3 \> k`, no further merges are performed.
// **Example 3:**
// **Input:** s = "yybyzybz", k = 2
// **Output:** "ybzybz"
// **Explanation:**
// * The characters `'y'` at indices 0 and 1 are close because `1 - 0 = 1 \<= k`. This pair has the smallest left index among all close pairs.
// * Remove the right `'y'`, resulting in `s = "ybyzybz"`.
// * The characters `'y'` at indices 0 and 2 are now close because `2 - 0 = 2 \<= k`.
// * Remove the right `'y'`, resulting in `s = "ybzybz"`.
// * No close pair remains, so no further merges are performed.
// **Constraints:**
// * `1 \<= s.length \<= 5 \* 105`
// * `1 \<= k \<= s.length`
// * `s` consists of lowercase English letters.

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn merge_characters(s: String, k: i32) -> String {
        let mut ans = String::new();
        let mut pos = vec![-k - 1; 128];
        for b in s.bytes() {
            let len = ans.len() as i32;
            let i = b as usize;
            if len > pos[i] + k {
                pos[i] = len;
                ans.push(b as char);
            }
        }
        ans
    }
}

struct Solution2;
impl Solution2 {
    pub fn merge_characters(s: String, k: i32) -> String {
        let (n, k) = (s.len(), k as usize);
        let (mut cnt, mut j) = (vec![vec![]; 26], 0);
        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            if cnt[idx].last().is_some_and(|&i| j <= i + k) {
                continue;
            }
            cnt[idx].push(j);
            j += 1;
        }
        let mut ans = vec![' '; j];
        for (idx, v) in cnt.iter().enumerate() {
            if v.is_empty() {
                continue;
            }
            let c = (b'a' + idx as u8) as char;
            for &i in v {
                ans[i] = c;
            }
        }
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_merge_characters_1() {
        assert_eq!(
            String::from("abc"),
            Solution::merge_characters(String::from("abca"), 3)
        );
    }
    #[test]
    pub fn test_merge_characters_2() {
        assert_eq!(
            String::from("abca"),
            Solution::merge_characters(String::from("aabca"), 2)
        );
    }
    #[test]
    pub fn test_merge_characters_3() {
        assert_eq!(
            String::from("ybzybz"),
            Solution::merge_characters(String::from("yybyzybz"), 2)
        );
    }
}
