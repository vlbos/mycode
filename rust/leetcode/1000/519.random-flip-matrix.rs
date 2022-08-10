/*
 * @lc app=leetcode id=519 lang=rust
 *
 * [519] Random Flip Matrix
 */

// @lc code=start
pub  struct  Solution {
s:std::collections::HashSet<i32>,
m:i32,
n:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(m: i32, n: i32) -> Self {
        Self{s:std::collections::HashSet::new(),m,n}
    }
    
    fn flip(&mut self) -> Vec<i32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let m = self.m;
        let n = self.n;
        let  b = m*n;
        loop{
             let xy = rng.gen_range(0,b);
             let x = xy/m;
             let y= xy%m;
             let i = x*n+y;
             if !self.s.contains(&i){
             self.s.insert(i);
             return vec![y,x];
            }
        }
        Vec::new()
    }
    
    fn reset(&mut self) {
        self.s.clear();
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(m, n);
 * let ret_1: Vec<i32> = obj.flip();
 * obj.reset();
 */
// @lc code=end

