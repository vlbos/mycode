/*
 * @lc app=leetcode id=1669 lang=rust
 *
 * [1669] Merge In Between Linked Lists
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
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut q = list1.clone();
        let mut p = &mut list1;
        for _ in 0..=b {
            q = q.unwrap().next;
        }
        for _ in 0..a - 1 {
            p = &mut p.as_mut().unwrap().next;
        }
        p.as_mut().unwrap().next = list2;
        while p.as_mut().unwrap().next.is_some() {
            p = &mut p.as_mut().unwrap().next;
        }
        p.as_mut().unwrap().next = q;

        list1
    }
}
// @lc code=end
