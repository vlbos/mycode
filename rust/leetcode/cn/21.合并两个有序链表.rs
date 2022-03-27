/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut q1 = l1;
        let mut q2 = l2;
        let mut h = Some(Box::new(ListNode::new(-1)));
        let mut c = &mut h;
        loop {
            match (q1.take(), q2.take()) {
                (Some(mut _q1), Some(mut _q2)) => {
                    if &_q1.val < &_q2.val {
                        let _n = _q1.next.take();
                        if let Some(ref mut _c) = c {
                            _c.next = Some(_q1);
                            c = &mut _c.next;
                        }
                        q1 = _n;
                        q2 = Some(_q2);
                    } else {
                        let _n = _q2.next.take();
                        if let Some(ref mut _c) = c {
                            _c.next = Some(_q2);
                            c = &mut _c.next;
                        }
                        q1 = Some(_q1);
                        q2 = _n;
                    }
                }
                (Some(_q1), None) => {
                    if let Some(ref mut _c) = c {
                        _c.next = Some(_q1);
                        break;
                    }
                }
                (None, Some(_q2)) => {
                    if let Some(ref mut _c) = c {
                        _c.next = Some(_q2);
                        break;
                    }
                }
                _ => break,
            }
        }

        h.unwrap().next
    }
}
// @lc code=end
