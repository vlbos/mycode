/*
 * @lc app=leetcode id=641 lang=rust
 *
 * [641] Design Circular Deque
 */

// @lc code=start
struct MyCircularDeque {
k:i32,
cq:Vec<i32>,
count:i32,
head:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        Self{k,cq:vec![0;k as usize],count:0,head:0}
    }
    
    fn insert_front(&mut self, value: i32) -> bool {
        if self.count==self.k{
        return false;
        }
        self.head= if self.head==0 {self.k-1} else{self.head-1};
        self.cq[self.head as usize]=value;
        self.count+=1;
        true
    }
    
    fn insert_last(&mut self, value: i32) -> bool {
        if self.count==self.k{
        return false;
        }
        let i=((self.head+self.count)%self.k) as usize;
        self.cq[i]=value;
        self.count+=1;
        true
    }
    
    fn delete_front(&mut self) -> bool {
        if self.count==0{
        return false;
        }
        self.head=if self.head<self.k-1{self.head+1}else{0};
        self.count-=1;
        true
    }
    
    fn delete_last(&mut self) -> bool {
        if self.count==0{
        return false;
        }
        self.count-=1;
        true
    }
    
    fn get_front(&self) -> i32 {
        if self.count==0{
        return -1;
        }
        self.cq[self.head as usize]
    }
    
    fn get_rear(&self) -> i32 {
        if self.count==0{
        return -1;
        }
        self.cq[((self.head+self.count-1)%self.k )as usize]
    }
    
    fn is_empty(&self) -> bool {
        self.count==0
    }
    
    fn is_full(&self) -> bool {
        self.count==self.k
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
// @lc code=end

