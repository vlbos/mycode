/*
 * @lc app=leetcode id=622 lang=rust
 *
 * [622] Design Circular Queue
 */

// @lc code=start
struct MyCircularQueue {
  q:Vec<i32>,
  h:usize,
  count:usize,
  k:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self{h:0,count:0,k ,q:vec![0;k ]}
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.count==self.k{
        return false;
        }
        self.q[(self.h+self.count)%self.k]=value;
        self.count+=1;
        true
    }
    
    fn de_queue(&mut self) -> bool {
        if self.count==0{
        return false;
        }
        self.h=(self.h+1)%self.k;
        self.count-=1;
        true
    }
    
    fn front(&self) -> i32 {
        if self.count==0{
        return -1;
        }
        self.q[self.h as usize]
    }
    
    fn rear(&self) -> i32 {
         if self.count==0{
           return -1;
        }
        self.q[(self.h+self.count-1)%self.k]
    }
    
    fn is_empty(&self) -> bool {
       self.count==0
    }
    
    fn is_full(&self) -> bool {
        self.count==self.k
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
// @lc code=end

