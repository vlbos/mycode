/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
        let mut inc = 0;
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut ans:Option<Box<ListNode>> = None;
        let mut p = &mut ans;
        while p1.is_some() && p2.is_some() {
            if let Some(n1) = p1 {
                if let Some(n2) = p2 {
                    let val = n1.val + n2.val + inc;
                    inc = val / 10;
                    if let Some(ref mut _p) = p {
                        _p.next = Some(Box::new(ListNode::new(val % 10)));
                        p = &mut _p.next;
                    } else {
                        ans = Some(Box::new(ListNode::new(val % 10)));
                        p = &mut ans;
                    }
                    p1 = &n1.next;
                    p2 = &n2.next;
                }
            }
        }
        while let Some(n1) = p1 {
            let val = n1.val + inc;
            inc = val / 10;

            if let Some(ref mut _p) = p {
                _p.next = Some(Box::new(ListNode::new(val % 10)));
                p = &mut _p.next;
            } else {
                ans = Some(Box::new(ListNode::new(val % 10)));
                p = &mut ans;
            }
            p1 = &n1.next;
        }
        while let Some(n2) = p2 {
            let val = n2.val + inc;
            inc = val / 10;
            if let Some(ref mut _p) = p {
                _p.next = Some(Box::new(ListNode::new(val % 10)));
                p = &mut _p.next;
            } else {
                ans = Some(Box::new(ListNode::new(val % 10)));
                p = &mut ans;
            }
            p2 = &n2.next;
        }
        if inc>0{
            if let Some(ref mut _p) = p {
                _p.next = Some(Box::new(ListNode::new(inc)));
            } 
        }
        ans
    }
}
// @lc code=end
