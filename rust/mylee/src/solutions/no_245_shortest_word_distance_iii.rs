// 245\. Shortest Word Distance III
// ================================

// Given a list of words and two words _word1_ and _word2_, return the shortest distance between these two words in the list.

// _word1_ and _word2_ may be the same and they represent two individual words in the list.

// **Example:**
// Assume that words = `["practice", "makes", "perfect", "coding", "makes"]`.

// **Input:** _word1_ = `“makes”`, _word2_ = `“coding”`
// **Output:** 1

// **Input:** _word1_ = `"makes"`, _word2_ = `"makes"`
// **Output:** 3

// **Note:**
// You may assume _word1_ and _word2_ are both in the list.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) 
#[allow(dead_code)] 
 struct Solution;
// @lc code=start
impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        // let mut i = -1 - (words.len() as i32);
        // let mut j = -1 - (words.len() as i32);
        // let mut dist = i32::max_value();
        // let is_same = word1 == word2;
        // for (k, w) in words.iter().enumerate() {
        //     if is_same && *w == word1 {
        //         if i < j {
        //             i = k as i32;
        //         } else {
        //             j = k as i32;
        //         }
        //     } else if *w == word1 {
        //         i = k as i32;
        //     } else if *w == word2 {
        //         j = k as i32;
        //     } else {
        //         continue;
        //     }
        //     dist = i32::min(dist, i32::abs(i - j));
        // }
        // dist
        let n = words.len() as i32;
        let (mut i1, mut i2) = (-n - 1, -n - 1);
        let mut ans = i32::MAX;
        for (j, w) in words.iter().enumerate() {
            if *w != word1 && *w != word2 {
                continue;
            }
            let j = j as i32;
            if word1 == word2 && *w == word1 {
                if i1 < i2 {
                    i1 = j;
                } else {
                    i2 = j;
                }
            } else if *w == word1 {
                i1 = j;
            } else if *w == word2 {
                i2 = j;
            }
            ans = ans.min((i1 - i2).abs());
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    fn test_shortest_distance_iii_1() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(
            Solution::shortest_word_distance(
                words,
                String::from("coding"),
                String::from("practice")
            ),
            3
        );
    }

    #[test]
    fn test_shortest_distance_iii_2() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(
            Solution::shortest_word_distance(words, String::from("makes"), String::from("coding")),
            1
        );
    }

    #[test]
    fn test_shortest_distance_iii_3() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(
            Solution::shortest_word_distance(words, String::from("makes"), String::from("makes")),
            3
        );
    }
}
