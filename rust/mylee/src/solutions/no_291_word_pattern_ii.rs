// 291\. Word Pattern II
// =====================

// Given a `pattern` and a string `str`, find if `str` follows the same pattern.

// Here **follow** means a full match, such that there is a bijection between a letter in `pattern` and a **non-empty** substring in `str`.

// **Example 1:**

// **Input:** pattern = `"abab"`, str = `"redblueredblue"`
// **Output:** true

// **Example 2:**

// **Input:** pattern = pattern = `"aaaa"`, str = `"asdasdasdasd"`
// **Output:** true

// **Example 3:**

// **Input:** pattern = `"aabb"`, str = `"xyzabcxzyabc"`
// **Output:** false

// **Notes:**
// You may assume both `pattern` and `str` contains only lowercase letters.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Dropbox](https://leetcode.ca/tags/#Dropbox) [Facebook](https://leetcode.ca/tags/#Facebook) [Pony.ai](https://leetcode.ca/tags/#Pony.ai) [Uber](https://leetcode.ca/tags/#Uber)
// use std::collections::{HashMap, HashSet};

// @lc code=start
impl Solution {
    pub fn word_pattern_match(pattern: String, str: String) -> bool {
        // let (pchars, pdict) = Solution::pattern_analyse(&pattern);
        // let chars = str.chars().collect::<Vec<char>>();
        // Solution::match_rec(&pchars, pdict, &chars, HashSet::new()).0
        use std::collections::HashMap;
        let mut m = HashMap::new();
        fn back_track(
            pattern: &String,
            str: &String,
            p: usize,
            r: usize,
            m: &mut HashMap<char, String>,
        ) -> bool {
            if p == pattern.len() && r == str.len() {
                return true;
            }
            if p == pattern.len() || r == str.len() {
                return false;
            }
            let c = pattern.chars().nth(p).unwrap();
            for i in r..str.len() {
                let t = str[r..=i].to_string();
                if let Some(v) = m.get(&c) {
                    if v == &t {
                        if back_track(pattern, str, p + 1, i + 1, m) {
                            return true;
                        }
                    }
                } else {
                    if m.values().all(|x| x != &t) {
                        m.insert(c, t);
                        if back_track(pattern, str, p + 1, i + 1, m) {
                            return true;
                        }
                        m.remove(&c);
                    }
                }
            }
            false
        }
        back_track(&pattern, &str, 0, 0, &mut m)
    }

    // // @return is_match, should pruning
    // fn match_rec(
    //     pchars: &[char],
    //     pdict: HashMap<char, Vec<char>>,
    //     chars: &[char],
    //     created: HashSet<Vec<char>>,
    // ) -> (bool, bool) {
    //     if pchars.is_empty() {
    //         return (chars.is_empty(), false);
    //     }
    //     let expect_len = pchars
    //         .iter()
    //         .fold(0usize, |sum, pc| pdict.get(pc).unwrap().len() + sum);
    //     if expect_len > chars.len() {
    //         return (false, true);
    //     }
    //     let curr_pc = pchars[0];
    //     let inner = pdict.get(&curr_pc).unwrap();
    //     if inner.is_empty() {
    //         for i in 1..=chars.len() {
    //             let to_create = chars[..i].to_vec();
    //             if created.contains(&to_create) {
    //                 continue;
    //             }
    //             let mut new_dict = pdict.clone();
    //             let mut new_created = created.clone();
    //             new_dict.insert(curr_pc, to_create.clone());
    //             new_created.insert(to_create);
    //             let res = Solution::match_rec(&pchars[1..], new_dict, &chars[i..], new_created);
    //             if res.0 {
    //                 return res;
    //             } else if res.1 {
    //                 break;
    //             }
    //         }
    //         (false, false)
    //     } else {
    //         if chars.len() < inner.len() || !Solution::chars_equal(&inner, &chars[..inner.len()]) {
    //             (false, false)
    //         } else {
    //             Solution::match_rec(&pchars[1..], pdict.clone(), &chars[inner.len()..], created)
    //         }
    //     }
    // }

    // // assume chars1.len() == chars2.len()
    // fn chars_equal(chars1: &[char], chars2: &[char]) -> bool {
    //     for i in 0..chars1.len() {
    //         if chars1[i] != chars2[i] {
    //             return false;
    //         }
    //     }
    //     return true;
    // }

    // fn pattern_analyse(pattern: &str) -> (Vec<char>, HashMap<char, Vec<char>>) {
    //     let chars = pattern.chars().collect::<Vec<char>>();
    //     let mut res = HashMap::new();
    //     for c in pattern.chars() {
    //         res.entry(c).or_insert_with(|| vec![]);
    //     }
    //     (chars, res)
    // }
}
// @lc code=end
#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_pattern_match1() {
        assert!(Solution::word_pattern_match(
            String::from("abab"),
            String::from("redblueredblue")
        ));
    }

    #[test]
    fn test_word_pattern_match2() {
        assert!(Solution::word_pattern_match(
            String::from("aaaa"),
            String::from("asdasdasdasd")
        ));
    }

    #[test]
    fn test_word_pattern_match3() {
        assert!(!Solution::word_pattern_match(
            String::from("aabb"),
            String::from("xyzabcxzyabc")
        ));
    }

    #[test]
    fn test_word_pattern_match4() {
        assert!(Solution::word_pattern_match(
            String::from("d"),
            String::from("e")
        ));
    }

    #[test]
    fn test_word_pattern_match5() {
        assert!(!Solution::word_pattern_match(
            String::from("ab"),
            String::from("aa")
        ));
    }
}
