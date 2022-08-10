// 161. One Edit Distance
// Given two strings s and t, determine if they are both one edit distance apart.

// Note:

// There are 3 possiblities to satisify one edit distance apart:

// Insert a character into s to get t
// Delete a character from s to get t
// Replace a character of s to get t
// Example 1:

// Input: s = "ab", t = "acb"
// Output: true
// Explanation: We can insert 'c' into s to get t.
// Example 2:

// Input: s = "cab", t = "ad"
// Output: false
// Explanation: We cannot get t from s by only one step.
// Example 3:

// Input: s = "1203", t = "1213"
// Output: true
// Explanation: We can replace '0' with '1' to get t.
// Difficulty:
// Medium
// Lock:
// Prime
// Company:
// Amazon Facebook Google Microsoft Snapchat Twitter Uber
#[allow(dead_code)]
pub  struct  Solution;
// @lc code=start

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        // let s_chars = s.chars().collect::<Vec<char>>();
        // let t_chars = t.chars().collect::<Vec<char>>();
        // let s_len = s.len();
        // let t_len = t.len();
        // if i32::abs((s_len as i32) - (t_len as i32)) > 1 {
        //     false
        // } else if s_len == t_len {
        //     let mut count = 0;
        //     for i in 0..s_len {
        //         if s_chars[i] != t_chars[i] {
        //             count += 1;
        //         }
        //     }
        //     count == 1
        // } else if s_len == 0 || t_len == 0 {
        //     true
        // } else {
        //     let mut i = 0;
        //     while i < s_len && i < t_len && s_chars[i] == t_chars[i] {
        //         i += 1;
        //     }
        //     let mut j = i + 1;
        //     let (longer_chars, shorter_chars) = if s_len > t_len {
        //         (&s_chars, &t_chars)
        //     } else {
        //         (&t_chars, &s_chars)
        //     };
        //     while i < shorter_chars.len() && j < longer_chars.len() {
        //         if shorter_chars[i] != longer_chars[j] {
        //             return false;
        //         }
        //         i += 1;
        //         j += 1;
        //     }
        //     true
        let (sn, tn) = (s.len() as i32, t.len() as i32);
        if (sn - tn).abs() > 1 {
            return false;
        }
        if sn == 0 || tn == 0 {
            return true;
        }
        if sn == tn {
            return s.chars().zip(t.chars()).filter(|(a, b)| a != b).count() == 1;
        }
        let (s, t) = if sn > tn { (t, s) } else { (s, t) };
        let (bs, bt) = (s.as_bytes(), t.as_bytes());
        for i in 0..bs.len() {
            if bs[i] != bt[i] && bs[i] != bt[i + 1] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_edit_distance_1() {
        assert!(Solution::is_one_edit_distance(
            String::from("ab"),
            String::from("acb")
        ));
    }
    #[test]
    fn test_is_one_edit_distance_2() {
        assert!(!Solution::is_one_edit_distance(
            String::from("cab"),
            String::from("ad"),
        ));
    }
    #[test]
    fn test_is_one_edit_distance_3() {
        assert!(Solution::is_one_edit_distance(
            String::from("1203"),
            String::from("1213")
        ));
    }
}
