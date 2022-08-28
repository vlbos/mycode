// 1989\. Maximum Number of People That Can Be Caught in Tag[](https://leetcode.ca/2021-09-03-1989-Maximum-Number-of-People-That-Can-Be-Caught-in-Tag/#1989-maximum-number-of-people-that-can-be-caught-in-tag)
// ============================================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-09-03-1989-Maximum-Number-of-People-That-Can-Be-Caught-in-Tag/#level)
// ------------------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-09-03-1989-Maximum-Number-of-People-That-Can-Be-Caught-in-Tag/#description)
// ------------------------------------------------------------------------------------------------------------------

// You are playing a game of tag with your friends. In tag, people are divided into two teams: people who are “it”, and people who are not “it”. The people who are “it” want to catch as many people as possible who are not “it”.

// You are given a **0-indexed** integer array `team` containing only zeros (denoting people who are not “it”) and ones (denoting people who are “it”), and an integer `dist`. A person who is “it” at index `i` can catch any one person whose index is in the range `[i - dist, i + dist]` (**inclusive**) and is **not** “it”.

// Return _the **maximum** number of people that the people who are “it” can catch_.

// **Example 1:**

// **Input:** team = \[0,1,0,1,0\], dist = 3

// **Output:** 2

// **Explanation:**

// The person who is “it” at index 1 can catch people in the range \[i-dist, i+dist\] = \[1-3, 1+3\] = \[-2, 4\].

// They can catch the person who is not “it” at index 2.

// The person who is “it” at index 3 can catch people in the range \[i-dist, i+dist\] = \[3-3, 3+3\] = \[0, 6\].

// They can catch the person who is not “it” at index 0.

// The person who is not “it” at index 4 will not be caught because the people at indices 1 and 3 are already catching one person.

// **Example 2:**

// **Input:** team = \[1\], dist = 1

// **Output:** 0

// **Explanation:**

// There are no people who are not “it” to catch.

// **Example 3:**

// **Input:** team = \[0\], dist = 1

// **Output:** 0

// **Explanation:**

// There are no people who are “it” to catch people.

// **Constraints:**

// *   `1 <= team.length <= 10^5`
// *   `0 <= team[i] <= 1`
// *   `1 <= dist <= team.length`

// Solution[](https://leetcode.ca/2021-09-03-1989-Maximum-Number-of-People-That-Can-Be-Caught-in-Tag/#solution)
// ------------------------------------------------------------------------------------------------------------

// Use a greedy approach. For each element 1, find the leftmost element 0 that is within distance `dist` of the element 1, and the element 0 is caught by the 1. Once an element 0 is caught by an element 1, it can no longer be caught again, so move on to the next element 0. If an element 1 cannot catch any element 0, then move on to the next element 1. Finally, return the number of elements 1 that can catch at least one element 0 each, which is the maximum number of people that can be caught in tag.

//     class Solution {
//         public int catchMaximumAmountofPeople(int[] team, int dist) {

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
