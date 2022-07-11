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

// Assume you have a dictionary and given a word, find whether its abbreviation is unique in the dictionary. A word's abbreviation is unique if no _other_ word from the dictionary has the same abbreviation.

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
use std::collections::HashMap;

struct ValidWordAbbr {
    values: HashMap<String, (usize, String)>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut dict: HashMap<String, (usize, String)> = HashMap::new();
        for s in dictionary {
            let abbr = ValidWordAbbr::to_abbr(&s);
            dict.entry(abbr)
                .and_modify(|c| {
                    if s != c.1 {
                        c.0 += 1;
                    }
                })
                .or_insert_with(|| (1, s));
        }
        Self { values: dict }
    }

    fn is_unique(&self, word: String) -> bool {
        self.values
            .get(&ValidWordAbbr::to_abbr(&word))
            .map_or(true, |c| c.0 == 1 && c.1 == word)
    }

    fn to_abbr(word: &str) -> String {
        if word.len() <= 2 {
            return word.to_string();
        }
        let chars = word.chars().collect::<Vec<char>>();
        let chars_last_i = chars.len() - 1;
        let mut res = String::new();
        res.push(chars[0]);
        res.extend(chars[1..chars_last_i].len().to_string().chars());
        res.push(chars[chars_last_i]);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    fn test_valid_word_abbr() {
        let input = map_to_string(&["deer", "door", "cake", "card"]);
        let validator = ValidWordAbbr::new(input);
        assert!(!validator.is_unique(String::from("dear")));
        assert!(validator.is_unique(String::from("cart")));
        assert!(!validator.is_unique(String::from("cane")));
        assert!(validator.is_unique(String::from("make")));
    }
}
