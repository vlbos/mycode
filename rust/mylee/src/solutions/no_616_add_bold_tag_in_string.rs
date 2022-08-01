// 616\. Add Bold Tag in String
// ============================

// Given a string **s** and a list of strings **dict**, you need to add a closed pair of bold tag `<b>` and `</b>` to wrap the substrings in s that exist in dict.
// If two such substrings overlap, you need to wrap them together by only one pair of closed bold tag.
// Also, if two substrings wrapped by bold tags are consecutive, you need to combine them.

// **Example 1:**

// **Input:**
// s = "abcxyz123"
// dict = \["abc","123"\]
// **Output:**
// "<b>abc</b>xyz<b>123</b>"

// **Example 2:**

// **Input:**
// s = "aaabbcc"
// dict = \["aaa","aab","bc"\]
// **Output:**
// "<b>aaabbc</b>c"

// **Note:**

// 1.  The given dict won't contain duplicates, and its length won't exceed 100.
// 2.  All the strings in input have length in range \[1, 1000\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
// @lc code=start
// const OPEN_TAG: &'static str = "<b>";
// const CLOSE_TAG: &'static str = "</b>";

impl Solution {
    pub fn add_bold_tag(s: String, dict: Vec<String>) -> String {
        // let mut ranges = vec![];
        // let len = s.len();
        // if dict.is_empty() {
        //     return s;
        // }
        // for d in dict {
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
        let mut pairs = Vec::new();
        for w in &dict {
            let i = s.find(w).unwrap();
            pairs.push(vec![i, i + w.len()]);
        }
        pairs.sort();
        let mut merged = Vec::new();
        let mut last = pairs[0].clone();
        let mut i = 1;
        while i < pairs.len() {
            if pairs[i][0] > last[1] {
                merged.push(last.clone());
                last = pairs[i].clone();
            } else {
                last[1] = pairs[i][1];
            }

            i += 1;
        }
        merged.push(last.clone());
        let mut s = s;
        for idx in merged.iter().rev() {
            s.insert_str(idx[1], "</b>");
            s.insert_str(idx[0], "<b>");
        }
        s
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_add_bold_1tag_1() {
        assert_eq!(
            Solution::add_bold_tag(String::from("abcxyz123"), lc_vec_s!["abc", "123"]),
            String::from("<b>abc</b>xyz<b>123</b>")
        );
    }

    #[test]
    fn test_add_bold_tag_2() {
        assert_eq!(
            Solution::add_bold_tag(String::from("aaabbcc"), lc_vec_s!["aaa", "aab", "bc"]),
            String::from("<b>aaabbc</b>c")
        );
    }
}
