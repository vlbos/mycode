/*
 * @lc app=leetcode id=82 lang=rust
 *
 * [82] Remove Duplicates from Sorted List II
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.clone().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;
        let mut result = None;
        let mut target = &mut result;
        let mut skip = false;
        if let Some(mut n) = head.take() {
            let mut q  = n.next.take();
            let mut p = n;
            while let Some(mut _q) = q.take() {
                q = _q.next.take();
                if _q.val == p.val {
                    skip = true;
                    continue;
                }

                if skip {
                    skip = false;
                    p = _q;
                } else {
                    target = &mut target.get_or_insert(p).next;
                    p = _q;
                }
            }
            if !skip {
                *target = Some(p);
            }
        }

        result
    }
}
// @lc code=end
