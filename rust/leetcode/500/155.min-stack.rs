/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
struct MinStack {
min:i32,
    s:Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
       Self {
              min:i32::MAX,
              s: Vec::<i32>::new()
            }
    }
    
    fn push(&mut self, val: i32) {
        if val <= self.min{
            self.s.push(self.min);
            self.min=val;
        }
       self.s.push(val);
    }
    
    fn pop(&mut self) {
       if !self.s.is_empty(){
            if let Some(v)= self.s.pop(){
                    if v==self.min {
                        if let Some(_min)=self.s.pop(){
                            self.min = _min;
                        }
                    }
            }
        }
    }
    
    fn top(&self) -> i32 {
        if self.s.is_empty(){
           return -1;
        }
        self.s[self.s.len()-1]
    }
    
    fn get_min(&self) -> i32 {
       if self.s.is_empty(){
           return -1;
        }
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end

