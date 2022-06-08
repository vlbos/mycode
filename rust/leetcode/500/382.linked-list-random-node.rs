/*
 * @lc app=leetcode id=382 lang=rust
 *
 * [382] Linked List Random Node
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
struct Solution {
head: Option<Box<ListNode>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        Self{head}
    }
    
    fn get_random(&self) -> i32 {
        use rand::Rng;
         let mut rng = rand::thread_rng();
        let mut p = &self.head;
        let mut count = 1;
        let mut ans = p.as_ref().unwrap().val;
        while let Some(n)=p{
             p=&n.next;
             if rng.gen::<i32>()%count==0{
                ans = n.val;
             }
             count+=1;
        }
        ans
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
// @lc code=end

