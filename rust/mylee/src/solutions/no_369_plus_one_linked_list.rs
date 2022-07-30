// 369\. Plus One Linked List
// ==========================

// Given a non-negative integer represented as **non-empty** a singly linked list of digits, plus one to the integer.

// You may assume the integer do not contain any leading zero, except the number 0 itself.

// The digits are stored such that the most significant digit is at the head of the list.

// **Example :**

// **Input:** \[1,2,3\]
// **Output:** \[1,2,4\]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Google](https://leetcode.ca/tags/#Google)

use crate::solutions::util::linked_list::ListNode;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// use std::borrow::BorrowMut;

impl Solution {
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let inc = match head {
        //     Some(ref mut head_node) => Solution::plus_one_recursive(head_node),
        //     None => false,
        // };
        // if inc {
        //     Some(Box::new({
        //         let mut new_head = ListNode::new(1);
        //         new_head.next = head;
        //         new_head
        //     }))
        // } else {
        //     head
        // }
        fn dfs(head: &mut Option<Box<ListNode>>) -> i32 {
            if let Some(node) = head {
                let mut ans = dfs(&mut node.next) + node.val;
                node.val = ans % 10;
                ans / 10
            } else {
                1
            }
        }
        if dfs(&mut head) > 0 {
            Some(Box::new(ListNode { val: 1, next: head }))
        } else {
            head
        }
    }

    // pub fn plus_one_recursive(mut head: &mut Box<ListNode>) -> bool {
    //     let next = head.next.borrow_mut();
    //     let inc = match next {
    //         Some(ref mut next_ref) => Solution::plus_one_recursive(next_ref),
    //         None => true,
    //     };
    //     if inc {
    //         head.val += 1;
    //     }
    //     if head.val == 10 {
    //         head.val = 0;
    //         true
    //     } else {
    //         false
    //     }
    // }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::linked;
    // use crate::solutions::util::linked_list::to_list;
    // use crate::solutions::util::linked_list::ListNode;
    #[test]
    fn test_plus_one() {
        let mut input = linked![1, 2, 3];
        let output = linked![1, 2, 4];
        input = Solution::plus_one(input);
        assert_eq!(input, output);
    }

    #[test]
    fn test_plus_one_1() {
        let mut input = linked![9];
        let output = linked![1, 0];
        input = Solution::plus_one(input);
        assert_eq!(input, output);
    }
}
