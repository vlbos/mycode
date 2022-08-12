// 408\. Valid Word Abbreviation
// =============================

// Given a **non-empty** string `s` and an abbreviation `abbr`, return whether the string matches with the given abbreviation.

// A string such as `"word"` contains only the following valid abbreviations:

// \["word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2", "2r1", "3d", "w3", "4"\]

// Notice that only the above abbreviations are valid abbreviations of the string `"word"`. Any other string is not a valid abbreviation of `"word"`.

// **Note:**
// Assume `s` contains only lowercase letters and `abbr` contains only lowercase letters and digits.

// **Example 1:**

// Given **s** = "internationalization", **abbr** = "i12iz4n":

// Return true.

// **Example 2:**

// Given **s** = "apple", **abbr** = "a2e":

// Return false.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
// @lc code=start

// #[derive(Debug)]
// enum AbbrType {
//     Char(char),
//     Num(i32),
// }

impl Solution {
    pub fn   valid_word_abbreviation(word: String, abbr: String) -> bool {
        let (bw, ba) = (word.as_bytes(), abbr.as_bytes());
        let (mut i, mut j) = (0, 0);
        while i < bw.len() && j < ba.len() {
            if (ba[j] as char).is_ascii_digit() {
                let k = j;
                while j < ba.len() && (ba[j] as char).is_ascii_digit() {
                    j += 1;
                }
                i += abbr[k..j].parse::<usize>().unwrap();
            }
            if j == ba.len() {
                return if i == bw.len() { true } else { false };
            }
            if i >= bw.len() {
                return false;
            }
            if bw[i] != ba[j] {
                return false;
            }
            i += 1;
            j += 1;
        }
        true
        // let mut achars: Vec<AbbrType> = vec![];
        // let mut current_num: Option<Vec<char>> = None;
        // for a in abbr.chars() {
        //     if a.is_digit(10) {
        //         current_num = match &mut current_num {
        //             Some(ref mut v) => {
        //                 v.push(a);
        //                 current_num
        //             }
        //             None => Some(vec![a]),
        //         }
        //     } else {
        //         if let Some(v) = current_num {
        //             let (num, valid_num) = Solution::validate_and_transform_num(v);
        //             if !valid_num {
        //                 return false;
        //             }
        //             achars.push(AbbrType::Num(num));
        //             current_num = None;
        //         }
        //         achars.push(AbbrType::Char(a));
        //     }
        // }
        // if let Some(v) = current_num {
        //     let (num, valid_num) = Solution::validate_and_transform_num(v);
        //     if !valid_num {
        //         return false;
        //     }
        //     achars.push(AbbrType::Num(num));
        // }
        // let mut i = 0;
        // let wchars = word.chars().collect::<Vec<char>>();
        // let wlen = wchars.len() as i32;
        // println!("{:?}", achars);
        // for a in achars {
        //     i += match a {
        //         AbbrType::Char(ch) => {
        //             if i >= wlen || ch != wchars[i as usize] {
        //                 return false;
        //             }
        //             1
        //         }
        //         AbbrType::Num(num) => {
        //             for j in 0..num {
        //                 if i + j >= wlen {
        //                     return false;
        //                 }
        //             }
        //             num
        //         }
        //     }
        // }
        // i == wlen
    }

    //pub fn  validate_and_transform_num(num_chars: Vec<char>) -> (i32, bool) {
    //     let str: String = num_chars.iter().collect();
    //     let num = str.parse::<i32>().unwrap();
    //     (num, num != 0 && num.to_string() == str)
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_valid_word_abbreviation_1() {
        assert!(Solution::valid_word_abbreviation(
            String::from("internationalization"),
            String::from("i12iz4n")
        ));
    }

    #[test]
   pub fn  test_valid_word_abbreviation_2() {
        assert!(!Solution::valid_word_abbreviation(
            String::from("apple"),
            String::from("a2e")
        ));
    }

    #[test]
   pub fn  test_valid_word_abbreviation_3() {
        assert!(!Solution::valid_word_abbreviation(
            String::from("hi"),
            String::from("1")
        ));
    }
}
