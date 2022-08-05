// 758\. Bold Words in String
// ==========================

// Given a set of keywords `words` and a string `S`, make all appearances of all keywords in `S` bold. Any letters between `<b>` and `</b>` tags become bold.

// The returned string should use the least number of tags possible, and of course the tags should form a valid combination.

// For example, given that `words = ["ab", "bc"]` and `S = "aabcd"`, we should return `"a<b>abc</b>d"`. Note that returning `"a<b>a<b>b</b>c</b>d"` would use more tags, so it is incorrect.

// **Note:**

// 1.  `words` has length in range `[0, 50]`.
// 2.  `words[i]` has length in range `[1, 10]`.
// 3.  `S` has length in range `[0, 500]`.
// 4.  All characters in `words[i]` and `S` are lowercase letters.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// const OPEN_TAG: &'static str = "<b>";
// const CLOSE_TAG: &'static str = "</b>";

impl Solution {
    pub fn bold_words(words: Vec<String>, s: String) -> String {
        // let mut ranges = vec![];
        // let len = s.len();
        // if words.is_empty() {
        //     return s;
        // }
        // for d in words {
        //     let dlen = d.len();
        //     if len >= dlen {
        //         for i in 0..(len - dlen + 1) {
        //             if &d == &s[i..(i + dlen)] {
        //                 ranges.push((i, i + dlen));
        //             }
        //         }
        //     }
        // }
        // ranges.sort();
        // let mut merged: Vec<(usize, usize)> = vec![];
        // for r in ranges {
        //     let last = merged.pop();
        //     if let None = last {
        //         merged.push(r);
        //     } else {
        //         let l = last.unwrap();
        //         if l.1 >= r.0 {
        //             merged.push((l.0, usize::max(l.1, r.1)));
        //         } else {
        //             merged.push(l);
        //             merged.push(r);
        //         }
        //     }
        // }
        // if merged.is_empty() {
        //     return s;
        // }

        // let mut res = String::new();
        // for i in 0..merged.len() {
        //     let curr = &merged[i];
        //     if i >= 1 {
        //         let last = &merged[i - 1];
        //         res += &s[last.1..curr.0];
        //     } else {
        //         res += &s[0..curr.0];
        //     }
        //     res += OPEN_TAG;
        //     res += &s[curr.0..curr.1];
        //     res += CLOSE_TAG;
        // }
        // if let Some(&(_, to)) = merged.last() {
        //     res += &s[to..];
        // }
        // res
        let n = s.len();
        let mut bold = vec![false; n + 2];
        for word in &words {
            let mut start = 0;
            while let Some(idx) = s[start..].find(word) {
                bold[start + idx + 1..=start + idx + word.len()].fill(true);
                start += idx + 1;
            }
        }
        let bs = s.as_bytes();
        let mut ans = String::new();
        for i in 1..=n {
            if bold[i] && !bold[i - 1] {
                ans.push_str("<b>")
            }
            ans.push(bs[i - 1] as char);
            if bold[i] && !bold[i + 1] {
                ans.push_str("</b>")
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_bold_words_1() {
        assert_eq!(
            Solution::bold_words(lc_vec_s!["abc", "123"], String::from("abcxyz123")),
            String::from("<b>abc</b>xyz<b>123</b>")
        );
    }

    #[test]
    fn test_bold_words_2() {
        assert_eq!(
            Solution::bold_words(lc_vec_s!["aaa", "aab", "bc"], String::from("aaabbcc")),
            String::from("<b>aaabbc</b>c")
        );
    }
}
