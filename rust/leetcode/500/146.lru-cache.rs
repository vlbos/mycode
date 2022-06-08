/*
 * @lc app=leetcode id=146 lang=rust
 *
 * [146] LRU Cache
 */

// @lc code=start
struct LRUCache {
    c:i32,
    t:i32,
    m:std::collections::HashMap<i32,(i32,i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self{c:capacity,t:0,m:std::collections::HashMap::new()}
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(mut n)=self.m.get_mut(&key){
            n.1=self.t;
            self.t+=1;
            n.0
        }
        else{-1}
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let mut v = self.m.entry(key).or_insert((value,self.t));
        v.0=value;
        v.1=self.t;
        self.t+=1;
        if self.m.len()>self.c as usize {
        let mut min = i32::MAX;

        let mut minkey=0;
        for (k,v) in &self.m{
            if v.1<min{
                min=v.1;
                minkey=*k;
            }
        }
        self.m.remove(&minkey);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

