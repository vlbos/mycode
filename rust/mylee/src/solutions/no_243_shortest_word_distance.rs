// 243\. Shortest Word Distance
// ============================

// Given a list of words and two words _word1_ and _word2_, return the shortest distance between these two words in the list.

// **Example:**
// Assume that words = `["practice", "makes", "perfect", "coding", "makes"]`.

// **Input:** _word1_ = `“coding”`, _word2_ = `“practice”`
// **Output:** 3

// **Input:** _word1_ = `"makes"`, _word2_ = `"coding"`
// **Output:** 1

// **Note:**
// You may assume that _word1_ **does not equal to** _word2_, and _word1_ and _word2_ are both in the list.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Oracle](https://leetcode.ca/tags/#Oracle) [Paypal](https://leetcode.ca/tags/#Paypal) [Uber](https://leetcode.ca/tags/#Uber) 
#[allow(dead_code)] 
 struct Solution;
// @lc code=start
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        // let mut i = -1 - (words.len() as i32);
        // let mut j = -1 - (words.len() as i32);
        // let mut dist = i32::max_value();
        // for (k, w) in words.iter().enumerate() {
        //     if *w == word1 {
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
        let (mut i, mut j) = (n, n);
        let mut ans = i32::MAX;
        for (k, w) in words.iter().enumerate() {
            if *w == word1 {
                i = k as i32;
                ans = ans.min((i - j).abs());
            } else if *w == word2 {
                j = k as i32;
                ans = ans.min((i - j).abs());
            }
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
    fn test_shortest_distance1() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(
            Solution::shortest_distance(words, String::from("coding"), String::from("practice")),
            3
        );
    }

    #[test]
    fn test_shortest_distance2() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(
            Solution::shortest_distance(words, String::from("makes"), String::from("coding")),
            1
        );
    }
}
