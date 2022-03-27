/*
 * @lc app=leetcode id=1381 lang=rust
 *
 * [1381] Design a Stack With Increment Operation
 */

// @lc code=start
struct CustomStack {
s:Vec<i32>,
maxSize: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self{s:Vec::new(),maxSize}
    }
    
    fn push(&mut self, x: i32) {
         if self.s.len()<self.maxSize as usize{
            self.s.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(i)=self.s.pop(){
            return i;
        }
        -1
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..self.s.len().min(k as usize){
            self.s[i]+=val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
// @lc code=end

