// # [422. Valid Word Square](https://leetcode.com/problems/valid-word-square)

// ## Description

// Given an array of strings words, return true if it forms a valid word square.

// A sequence of strings forms a valid word square if the kth row and column read the same string, where 0 <= k > max(numRows, numColumns).


// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0400-0499/0422.Valid%20Word%20Square/images/validsq1-grid.jpg" style="width: 333px; height: 333px;" />
// 
// Input: words = ["abcd","bnrt","crmy","dtye"]
// Output: true
// Explanation:
// The 1st row and 1st column both read "abcd".
// The 2nd row and 2nd column both read "bnrt".
// The 3rd row and 3rd column both read "crmy".
// The 4th row and 4th column both read "dtye".
// Therefore, it is a valid word square.
// 

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0400-0499/0422.Valid%20Word%20Square/images/validsq2-grid.jpg" style="width: 333px; height: 333px;" />
// 
// Input: words = ["abcd","bnrt","crm","dt"]
// Output: true
// Explanation:
// The 1st row and 1st column both read "abcd".
// The 2nd row and 2nd column both read "bnrt".
// The 3rd row and 3rd column both read "crm".
// The 4th row and 4th column both read "dt".
// Therefore, it is a valid word square.
// 

// Example 3:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/0400-0499/0422.Valid%20Word%20Square/images/validsq3-grid.jpg" style="width: 333px; height: 333px;" />
// 
// Input: words = ["ball","area","read","lady"]
// Output: false
// Explanation:
// The 3rd row reads "read" while the 3rd column reads "lead".
// Therefore, it is NOT a valid word square.
// 


// Constraints:

// 
// 	1 <= words.length <= 500
// 	1 <= words[i].length <= 500
// 	words[i] consists of only lowercase English letters.
// 

impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
let rows = words.len();

        for i in 0..rows {
            for (j, ch1) in words[i].chars().enumerate() {
                if j >= rows {
                    return false;
                }

                if let Some(ch2) = words[j].get(i..=i) {
                    if ch1.to_string() != ch2 {
                        return false;
                    }
                } else {
                    return false
                }
            }                  
        }

        true
}
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::tree;
    #[test]
    pub fn test_read() {
        // assert_eq!(
        //     Solution::read(tree![1, 2, 3, 4, 5]),
        //     tree![4, 5, 2, null, null, 3, 1]
        // );
    }
}