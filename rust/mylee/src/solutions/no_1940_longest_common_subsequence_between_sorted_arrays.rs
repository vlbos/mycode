// 1940\. Longest Common Subsequence Between Sorted Arrays[](https://leetcode.ca/2021-08-08-1940-Longest-Common-Subsequence-Between-Sorted-Arrays/#1940-longest-common-subsequence-between-sorted-arrays)
// ======================================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-08-1940-Longest-Common-Subsequence-Between-Sorted-Arrays/#level)
// ----------------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-08-08-1940-Longest-Common-Subsequence-Between-Sorted-Arrays/#description)
// ----------------------------------------------------------------------------------------------------------------

// Given an array of integer arrays `arrays` where each `arrays[i]` is sorted in **strictly increasing** order, return _an integer array representing the **longest common subsequence** between **all** the arrays_.

// A **subsequence** is a sequence that can be derived from another sequence by deleting some elements (possibly none) without changing the order of the remaining elements.

// **Example 1:**

//     Input: arrays = [[1,3,4],
//                      [1,4,7,9]]
//     Output: [1,4]
//     Explanation: The longest common subsequence in the two arrays is [1,4].
    

// **Example 2:**

//     Input: arrays = [[2,3,6,8],
//                      [1,2,3,5,6,7,10],
//                      [2,3,4,6,9]]
//     Output: [2,3,6]
//     Explanation: The longest common subsequence in all three arrays is [2,3,6].
    

// **Example 3:**

//     Input: arrays = [[1,2,3,4,5],
//                      [6,7,8]]
//     Output: []
//     Explanation: There is no common subsequence between the two arrays.
    

// **Constraints:**

// *   `2 <= arrays.length <= 100`
// *   `1 <= arrays[i].length <= 100`
// *   `1 <= arrays[i][j] <= 100`
// *   `arrays[i]` is sorted in strictly increasing order.

// Solution[](https://leetcode.ca/2021-08-08-1940-Longest-Common-Subsequence-Between-Sorted-Arrays/#solution)
// ----------------------------------------------------------------------------------------------------------

// Any element in the longest common subsequence must be in all of the arrays. Since all arrays are sorted in strictly increasing order, the same element does not exist more than once in the same array. Loop over all arrays and use a hash map to store each number’s number of occurrences in all arrays.

// Then loop over the shortest array among all arrays, and obtain the elements that have numbers of occurrences equal to the number of arrays. These elements form the longest common subsequence.

//     class Solution {
//         public List<Integer> longestCommomSubsequence(int[][] arrays) {


#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
