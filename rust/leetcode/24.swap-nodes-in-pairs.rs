/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
 */

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
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
       let mut result = None;
        let mut target = &mut result;

        while let Some(mut node) = head {
            if let Some(mut next) = node.next.take() {
                head = next.next.take();
                target = &mut target.get_or_insert(next).next.get_or_insert(node).next;
            } else {
                *target = Some(node);

                break;
            }
        }
        result

    }
}
// @lc code=end

