/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
 s1:Vec<i32>,s2:Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{s1:Vec::<i32>::new(),s2:Vec::<i32>::new()}
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        while let Some(v) = self.s1.pop(){
            self.s2.push(v);
        }
        self.s1.push(x);
        while let Some(v) = self.s2.pop(){
            self.s1.push(v);
        }
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
       if let Some(v) = self.s1.pop(){  
            return v;
      }
      0
    }
    
    /** Get the front element. */
    fn peek(&self) -> i32 {
        if self.s1.is_empty(){
        return 0;
        }
        self.s1[self.s1.len()-1]
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

