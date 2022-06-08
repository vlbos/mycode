/*
 * @lc app=leetcode id=445 lang=rust
 *
 * [445] Add Two Numbers II
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        let mut p = &l1;
        while let Some(n) = p {
            s1.push(n.val);
            p = &n.next;
        }
        p = &l2;
        while let Some(n) = p {
            s2.push(n.val);
            p = &n.next;
        }
        let mut next: Option<Box<ListNode>> = None;
        let mut carry = 0;
        while !s1.is_empty() || !s2.is_empty() {
            match (s1.pop(), s2.pop()) {
                (Some(v1), Some(v2)) => {
                    let mut val = v1 + v2 + carry;
                    carry = val / 10;
                    val %= 10;
                    next = Some(Box::new(ListNode { next, val }));
                }
                (None, Some(v)) => {
                    let mut val = v + carry;
                    carry = val / 10;
                    val %= 10;
                    next = Some(Box::new(ListNode { next, val }));
                }
                (Some(v), None) => {
                    let mut val = v + carry;
                    carry = val / 10;
                    val %= 10;
                    next = Some(Box::new(ListNode { next, val }));
                }
                _ => (),
            }
        }
        if carry>0{
        next = Some(Box::new(ListNode { next, val:carry }));
        }
        next
    }
}
// @lc code=end
