/*
 * @lc app=leetcode.cn id=933 lang=rust
 *
 * [933] 最近的请求次数
 */

// @lc code=start
struct RecentCounter {
    v:Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self{v:Vec::new()}
    }
    
    fn ping(&mut self, t: i32) -> i32 {
           self.v.insert(0,t);
           while self.v.len()>0 && self.v[self.v.len()-1]<t-3000{
                    self.v.pop();
           }
           self.v.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end

