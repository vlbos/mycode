/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge(lists: &mut Vec<Option<Box<ListNode>>>, l: usize, r: usize)  -> Option<Box<ListNode>> {
            let merge_two_lists = |mut a: Option<Box<ListNode>>,
                                   mut b: Option<Box<ListNode>>|
             -> Option<Box<ListNode>> {
                if a.is_none() || b.is_none() {
                    return if a.is_some() { a } else { b };
                }
                let mut head = ListNode::new(0);
                let mut tail = &mut head;
                while let (Some(na), Some(nb)) = (a.as_ref(), b.as_ref()) {
                    if na.val <= nb.val {
                        tail.next = a.take();
                        tail = tail.next.as_mut().unwrap();
                        a = tail.next.take();
                    } else {
                        tail.next = b.take();
                        tail = tail.next.as_mut().unwrap();
                        b = tail.next.take();
                    }
                   
                }
                tail.next = if a.is_some() { a } else { b };
                head.next
            };
            if l == r {
                return lists[l].take();
            }
            if l > r {
                return None;
            }
            let mid = (l + r) / 2;
           let left= merge(lists, l, mid);
            let right=merge(lists, mid + 1, r);
            merge_two_lists(left,right )
        }
        if lists.is_empty() {
            return None;
        }
        let n = lists.len() - 1;
        let mut lists=lists;
        merge(&mut lists, 0, n)
    }
}
// @lc code=end
