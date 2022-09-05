// # [2184. Number of Ways to Build Sturdy Brick Wall](https://leetcode.com/problems/number-of-ways-to-build-sturdy-brick-wall)

// ## Description

// You are given integers height and width which specify the dimensions of a brick wall you are building. You are also given a 0-indexed array of unique integers bricks, where the ith brick has a height of 1 and a width of bricks[i]. You have an infinite supply of each type of brick and bricks may not be rotated.

// Each row in the wall must be exactly width units long. For the wall to be sturdy, adjacent rows in the wall should not join bricks at the same location, except at the ends of the wall.

// Return the number of ways to build a sturdy wall. Since the answer may be very large, return it modulo 109 + 7.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2184.Number%20of%20Ways%20to%20Build%20Sturdy%20Brick%20Wall/images/image-20220220190749-1.png" style="width: 919px; height: 250px;" />
//
// Input: height = 2, width = 3, bricks = [1,2]
// Output: 2
// Explanation:
// The first two walls in the diagram show the only two ways to build a sturdy brick wall.
// Note that the third wall in the diagram is not sturdy because adjacent rows join bricks 2 units from the left.
//

// Example 2:

//
// Input: height = 1, width = 1, bricks = [5]
// Output: 0
// Explanation:
// There are no ways to build a sturdy wall because the only type of brick we have is longer than the width of the wall.
//

// Constraints:

//
// 	1 <= height <= 100
// 	1 <= width <= 10
// 	1 <= bricks.length <= 10
// 	1 <= bricks[i] <= 10
// 	All the values of bricks are unique.
//
// int buildWall(int height, int width, vector<int>& bricks) {

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
