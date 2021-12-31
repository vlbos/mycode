/*
 * @lc app=leetcode id=1656 lang=rust
 *
 * [1656] Design an Ordered Stream
 */

// @lc code=start
struct OrderedStream {
    d:Vec<String>,
    index:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        Self{d:vec![String::new();n as usize],index:0}
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.d[id_key as usize-1]=value;
        let mut ii = self.index;
        let mut flag = false;
        for i in  self.index..self.d.len(){
                if self.d[i].is_empty(){
                    break;
                }
                else{
                    flag = true;
                }
                ii=i;
        }
        if flag{
            let oindex = self.index;
            self.index=ii+1;
            return (&self.d[oindex..self.index]).to_vec();
        }
        vec![]
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */
// @lc code=end

