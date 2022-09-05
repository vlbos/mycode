// # [2340. Minimum Adjacent Swaps to Make a Valid Array](https://leetcode.com/problems/minimum-adjacent-swaps-to-make-a-valid-array)

// ## Description

// You are given a 0-indexed integer array nums.

// Swaps of adjacent elements are able to be performed on nums.

// A valid array meets the following conditions:

//
// 	The largest element (any of the largest elements if there are multiple) is at the rightmost position in the array.
// 	The smallest element (any of the smallest elements if there are multiple) is at the leftmost position in the array.
//

// Return the minimum swaps required to make nums a valid array.

// Example 1:

//
// Input: nums = [3,4,5,5,3,1]
// Output: 6
// Explanation: Perform the following swaps:
// - Swap 1: Swap the 3rd and 4th elements, nums is then [3,4,5,3,5,1].
// - Swap 2: Swap the 4th and 5th elements, nums is then [3,4,5,3,1,5].
// - Swap 3: Swap the 3rd and 4th elements, nums is then [3,4,5,1,3,5].
// - Swap 4: Swap the 2nd and 3rd elements, nums is then [3,4,1,5,3,5].
// - Swap 5: Swap the 1st and 2nd elements, nums is then [3,1,4,5,3,5].
// - Swap 6: Swap the 0th and 1st elements, nums is then [1,3,4,5,3,5].
// It can be shown that 6 swaps is the minimum swaps required to make a valid array.
//

// Example 2:

//
// Input: nums = [9]
// Output: 0
// Explanation: The array is already valid, so we return 0.
//

// Constraints:

//
// 	1 <= nums.length <= 105
// 	1 <= nums[i] <= 105
//
//  int minimumSwaps(vector<int>& nums) {

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
