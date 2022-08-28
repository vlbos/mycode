// #  Median of a Row Wise Sorted Matrix
// ================================

// In this article we will solve the most asked coding interview problem: Median of Row Wise Sorted Matrix

// **Problem Statement:** Given a row-wise sorted matrix of size r\*c, where r is no. of rows and c is no. of columns, find the median in the given matrix.

// **Assume** – r\*c is odd

// **Examples:**

// **Example 1:**
// **Input:**
// r = 3 , c = 3
// 1 4 9
// 2 5 6
// 3 8 7
// **Output:** Median = 5
// **Explanation:** If we find the linear sorted array, the array becomes 1 2 3 4 5 6 7 8 9
// So, median = 5

// **Example 2:**
// **Input:**
// r = 3 , c = 3
// 1 3 8
// 2 3 4
// 1 2 5
// **Output:** Median = 3
// **Explanation:** If we find the linear sorted array, the array becomes 1 1 2 2 3 3 4 5 7 8
// So, median = 3

// ### **Solution**

// **_Disclaimer_**: _Don’t jump directly to the solution, try it out yourself first._

// **Solution 1:**

// **Approach 1**: Brute Force Approach

// The approach is very simple, just fill all elements in a linear array sort the array using the sort function, and then find the middle value (**Median)**.

// For Eg,

// For the given matrix

// 1 3 8

// 2 3 4

// 1 2 5

// We find the sorted linear array as 1 1 2 2 3 3 4 5 8

// Now, the middle element of the array is 3.

// int Findmedian(int arr[3][3], int row, int col)

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
