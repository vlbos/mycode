/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 */

// @lc code=start
use std::collections::VecDeque;
struct MyStack {
q1:VecDeque<i32>,
        q2:VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{q1:VecDeque::<i32>::new(),q2:VecDeque::<i32>::new()}
    }
    
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.q2.push_back(x);
        while !self.q1.is_empty(){
            self.q2.push_back(self.q1.pop_front().unwrap());
        }
        while !self.q2.is_empty(){
            self.q1.push_back(self.q2.pop_front().unwrap());
        }
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.q1.is_empty() {
            return 0;
        }
        self.q1.pop_front().unwrap()
    }
    
    /** Get the top element. */
    fn top(&mut self) -> i32 {
        if self.q1.is_empty() {
        return 0;
        }
        *self.q1.front().unwrap()
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.q1.is_empty() 
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

