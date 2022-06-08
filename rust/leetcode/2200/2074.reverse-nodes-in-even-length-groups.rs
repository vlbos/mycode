/*
 * @lc app=leetcode id=2074 lang=rust
 *
 * [2074] Reverse Nodes in Even Length Groups
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
fn rev_even(mut before: &mut Box<ListNode>, len: i32) -> Option<&mut Box<ListNode>> {
    let mut prev = None;
    let mut node = before.next.take();

    let mut i = 0;
    while let Some(mut p) = node.take() {
        let next = p.next.take();
        p.next = prev;

        prev = Some(p);
        node = next;

        i += 1;
        if i == len {
            break;
        }
    }
    before.next = prev;

    if let Some(p) = advance(before, i) {
        p.next = node;
        return Some(p);
    }
    None
}

// Advance node by the specified distance
fn advance(p: &mut Box<ListNode>, len: i32) -> Option<&mut Box<ListNode>> {
    let mut node = p.next.as_mut();

    for i in 1..len {
        if let Some(p) = node {
            node = p.next.as_mut();
        } else {
            return None;
        }
    }

    node
}

// Return the length of the current group
// node - first node in the group
// maxlen - maximum length of the current group
fn get_len(mut node: &Option<Box<ListNode>>, maxlen: i32) -> i32 {
    let mut len = 0;
    while let Some(p) = node {
        len += 1;
        node = &p.next;

        if len == maxlen {
            break;
        }
    }

    len
}
impl Solution {
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = 1;
        let mut head = head;
        let mut pre = head.as_mut();
        while let Some(p) = pre {
            i += 1;
            let len = get_len(&p.next, i);

            if (len & 1) > 0 {
                pre = advance(p, i);
            } else {
                pre = rev_even(p, i);
            }
        }
        head
    }
}
// @lc code=end
