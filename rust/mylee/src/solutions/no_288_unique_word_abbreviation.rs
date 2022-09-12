// 288\. Unique Word Abbreviation
// ==============================

// An abbreviation of a word follows the form <first letter><number><last letter>. Below are some examples of word abbreviations:

// a) it                      --> it    (no abbreviation)

//      1
//      ↓
// b) d|o|g                   --> d1g

//               1    1  1
//      1---5----0----5--8
//      ↓   ↓    ↓    ↓  ↓
// c) i|nternationalizatio|n  --> i18n

//               1
//      1---5----0
//      ↓   ↓    ↓
// d) l|ocalizatio|n          --> l10n

// Assume you have a dictionary and given a word, find whether its abbreviation is unique in the dictionary.
//  A word's abbreviation is unique if no _other_ word from the dictionary has the same abbreviation.

// **Example:**

// Given dictionary = \[ "deer", "door", "cake", "card" \]

// isUnique("dear") -> `false`
// isUnique("cart") -> `true`
// isUnique("cane") -> `false`
// isUnique("make") -> `true`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
use std::collections::HashSet;

pub struct ValidWordAbbr {
    // values: HashMap<String, (usize, String)>,
    dic: HashSet<String>,
}

impl ValidWordAbbr {
    pub fn new(dictionary: Vec<String>) -> Self {
        // let mut dict: HashMap<String, (usize, String)> = HashMap::new();
        // for s in dictionary {
        //     let abbr = ValidWordAbbr::to_abbr(&s);
        //     dict.entry(abbr)
        //         .and_modify(|c| {
        //             if s != c.1 {
        //                 c.0 += 1;
        //             }
        //         })
        //         .or_insert_with(|| (1, s));
        // }
        // Self { values: dict }

        let mut dic = HashSet::new();
        for s in &dictionary {
            let a = if s.len() < 3 {
                s.clone()
            } else {
                format!("{}{}{}", &s[..1], s.len() - 2, &s[s.len() - 1..])
            };
            dic.insert(a.clone());
        }
        Self { dic }
    }

    pub fn is_unique(&self, word: String) -> bool {
        // self.values
        //     .get(&ValidWordAbbr::to_abbr(&word))
        //     .map_or(true, |c| c.0 == 1 && c.1 == word)
        let a = if word.len() < 3 {
            word.clone()
        } else {
            format!(
                "{}{}{}",
                &word[..1],
                word.len() - 2,
                &word[word.len() - 1..]
            )
        };

        !self.dic.contains(&a)
    }

    // pub fn    to_abbr(word: &str) -> String {
    //     if word.len() <= 2 {
    //         return word.to_string();
    //     }
    //     let chars = word.chars().collect::<Vec<char>>();
    //     let chars_last_i = chars.len() - 1;
    //     let mut res = String::new();
    //     res.push(chars[0]);
    //     res.extend(chars[1..chars_last_i].len().to_string().chars());
    //     res.push(chars[chars_last_i]);
    //     res
    // }
}
// @lc code=end
// use std::collections::{HashMap, HashSet};

// struct ValidWordAbbr {
//     data: HashMap<String, HashSet<String>>,
// }

// impl ValidWordAbbr {
//     fn new(dictionary: Vec<String>) -> Self {
//         let mut data = HashMap::new();

//         for d in dictionary {
//             if d.len() <= 2 {
//                 data.entry(d.clone()).or_insert(HashSet::new()).insert(d);
//             } else {
//                 data.entry(format!("{}{}{}", &d[0..1], d.len() - 2, &d[d.len() - 1..]))
//                     .or_insert(HashSet::new())
//                     .insert(d);
//             }
//         }

//         Self { data }
//     }

//     fn is_unique(&self, word: String) -> bool {
//         let w = if word.len() <= 2 {
//             word.clone()
//         } else {
//             format!(
//                 "{}{}{}",
//                 &word[0..1],
//                 word.len() - 2,
//                 &word[word.len() - 1..]
//             )
//         };

//         if let Some(d) = self.data.get(&w) {
//             d.contains(&word) && d.len() == 1
//         } else {
//             true
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;
    // ["ValidWordAbbr","isUnique","isUnique","isUnique","isUnique","isUnique"]
    // [[["deer","door","cake","card"]],["dear"],["cart"],["cane"],["make"],["cake"]]
    #[test]
    pub fn test_valid_word_abbr() {
        let input = map_to_string(&["deer", "door", "cake", "card"]);
        let validator = ValidWordAbbr::new(input);
        assert!(!validator.is_unique(String::from("dear")));
        assert!(validator.is_unique(String::from("cart")));
        assert!(!validator.is_unique(String::from("cane")));
        assert!(validator.is_unique(String::from("make")));
    }
}
