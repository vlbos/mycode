// # [2052. Minimum Cost to Separate Sentence Into Rows](https://leetcode.com/problems/minimum-cost-to-separate-sentence-into-rows)

// ## Description

// You are given a string sentence containing words separated by spaces, and an integer k.
// Your task is to separate sentence into rows where the number of characters in each row is at most k.
//  You may assume that sentence does not begin or end with a space, and the words in sentence are separated by a single space.

// You can split sentence into rows by inserting line breaks between words in sentence.
// A word cannot be split between two rows. Each word must be used exactly once, and the word order cannot be rearranged.
//  Adjacent words in a row should be separated by a single space, and rows should not begin or end with spaces.

// The cost of a row with length n is (k - n)^2, and the total cost is the sum of the costs for all rows except the last one.

//
// 	For example if sentence = "i love leetcode" and k = 12:
//
//     	Separating sentence into "i", "love", and "leetcode" has a cost of (12 - 1)^2 + (12 - 4)^2 = 185.
//     	Separating sentence into "i love", and "leetcode" has a cost of (12 - 6)^2 = 36.
//     	Separating sentence into "i", and "love leetcode" is not possible because the length of "love leetcode" is greater than k.
//
//
//

// Return the minimum possible total cost of separating sentence into rows.

// Example 1:

//
// Input: sentence = "i love leetcode", k = 12
// Output: 36
// Explanation:
// Separating sentence into "i", "love", and "leetcode" has a cost of (12 - 1)^2 + (12 - 4)^2 = 185.
// Separating sentence into "i love", and "leetcode" has a cost of (12 - 6)^2 = 36.
// Separating sentence into "i", "love leetcode" is not possible because "love leetcode" has length 13.
// 36 is the minimum possible total cost so return it.
//

// Example 2:

//
// Input: sentence = "apples and bananas taste great", k = 7
// Output: 21
// Explanation
// Separating sentence into "apples", "and", "bananas", "taste", and "great" has a cost of (7 - 6)^2 + (7 - 3)^2 + (7 - 7)^2 + (7 - 5)^2 = 21.
// 21 is the minimum possible total cost so return it.
//

// Example 3:

//
// Input: sentence = "a", k = 5
// Output: 0
// Explanation:
// The cost of the last row is not included in the total cost, and since there is only one row, return 0.
//

// Constraints:

//
// 	1 <= sentence.length <= 5000
// 	1 <= k <= 5000
// 	The length of each word in sentence is at most k.
// 	sentence consists of only lowercase English letters and spaces.
// 	sentence does not begin or end with a space.
// 	Words in sentence are separated by a single space.
//

//     int minimum_cost(string sentence, int k) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_cost(sentence: String, k: i32) -> i32 {
        if sentence.len() <= k as usize {
            return 0;
        }
        let words: Vec<&str> = sentence.split_ascii_whitespace().collect();
        let len = words.len();

        let mut dp = vec![0; len + 1];
        for i in 1..=len {
            let mut n = words[i - 1].len() as i32;
            dp[i] = dp[i - 1] + (k - n) * (k - n);
            for j in (1..i).rev() {
                n += words[j - 1].len() as i32 + 1;
                if n > k {
                    break;
                }
                dp[i] = dp[i].min(dp[j - 1] + (k - n) * (k - n));
            }
        }
        let mut last_row_len = words.last().unwrap().len();
        let mut i = words.len() - 2;
        while i > 0 && last_row_len + words[i].len() + 1 <= k as usize {
            last_row_len += words[i - 1].len() + 1;
            i -= 1;
        }
        *dp[i + 1..].iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_cost_1() {
        assert_eq!(
            36,
            Solution::minimum_cost("i love leetcode".to_string(), 12)
        );
    }
    #[test]
    pub fn test_minimum_cost_2() {
        assert_eq!(
            21,
            Solution::minimum_cost("apples and bananas taste great".to_string(), 7)
        );
    }
    #[test]
    pub fn test_minimum_cost_3() {
        assert_eq!(0, Solution::minimum_cost("a".to_string(), 5));
    }
}
