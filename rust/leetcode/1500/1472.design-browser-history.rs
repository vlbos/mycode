/*
 * @lc app=leetcode id=1472 lang=rust
 *
 * [1472] Design Browser History
 */

// @lc code=start
struct BrowserHistory {
h:Vec<String>,
curr:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        Self{h:vec![homepage.clone()],curr:0}
    }
    
    fn visit(&mut self, url: String) {
        self.curr+=1;
        self.h.truncate(self.curr);
        self.h.push(url);
    }
    
    fn back(&mut self, steps: i32) -> String {
        let steps=steps as usize;
        self.curr = if steps>self.curr{0}else{self.curr-steps};
        self.h[self.curr].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let f=self.curr+steps as usize;
        let n1 = self.h.len()-1;
        self.curr =  if f>n1{n1}else{f};
        self.h[self.curr].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
// @lc code=end

