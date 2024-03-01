// # [3063. Linked List Frequency](https://leetcode.com/problems/linked-list-frequency)

// ## Description

// Given the head of a linked list containing k distinct elements,
// return a linked list of length k containing the frequency of each distinct element in the given linked list in any order.

// Example 1:

// Input:   head = [1,1,1,2,2,3]

// Output:   [3,2,1]

// Explanation:  There are 3 distinct elements in the list.
// The frequency of 1 is 3, the frequency of 2 is 2 and the frequency of 3 is 1. Hence, we return 3 -> 2 -> 1.

// Note that 1 -> 2 -> 3, 1 -> 3 -> 2, 2 -> 1 -> 3, 2 -> 3 -> 1, and 3 -> 1 -> 2 are also valid answers.

// Example 2:

// Input:   head = [1,1,2,2,2]

// Output:   [2,3]

// Explanation:  There are 2 distinct elements in the list.
// The frequency of 1 is 2 and the frequency of 2 is 3. Hence, we return 2 -> 3.

// Example 3:

// Input:   head = [6,5,4,3,2,1]

// Output:   [1,1,1,1,1,1]

// Explanation:  There are 6 distinct elements in the list.
// The frequency of each of them is 1. Hence, we return 1 -> 1 -> 1 -> 1 -> 1 -> 1.

// Constraints:

// 	The number of nodes in the list is in the range [1, 105].
// 	1  <= Node.val  <= 105

// ```cpp
// /**
//  * Definition for singly-linked list.
//  * struct ListNode {
//  *     int val;
//  *     ListNode *next;
//  *     ListNode() : val(0), next(nullptr) {}
//  *     ListNode(int x) : val(x), next(nullptr) {}
//  *     ListNode(int x, ListNode *next) : val(x), next(next) {}
//  * };
//  */
// class Solution {
// public:
//     ListNode* frequencies_of_elements(ListNode* head) {

use crate::solutions::util::linked_list::ListNode;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub  struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//  pub fn  new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// use std::borrow::BorrowMut;

impl Solution {
    pub fn frequencies_of_elements(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::linked;
    #[test]
    pub fn test_frequencies_of_elements_1() {
        assert_eq!(
            linked![3, 2, 1],
            Solution::frequencies_of_elements(linked![1, 1, 1, 2, 2, 3])
        );
    }

    #[test]
    pub fn test_frequencies_of_elements_2() {
        assert_eq!(
            linked![2, 3],
            Solution::frequencies_of_elements(linked![1, 1, 2, 2, 2])
        );
    }
    #[test]
    pub fn test_frequencies_of_elements_3() {
        assert_eq!(
            linked![1, 1, 1, 1, 1, 1],
            Solution::frequencies_of_elements(linked![6, 5, 4, 3, 2, 1])
        );
    }
}
