/*
 * @lc app=leetcode id=981 lang=rust
 *
 * [981] Time Based Key-Value Store
 */

// @lc code=start
struct TimeMap {
m:std::collections::HashMap<String,std::collections::BTreeMap<i32,String>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self{m:std::collections::HashMap::new()}
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.m.entry(key).or_insert(std::collections::BTreeMap::new()).insert(timestamp,value);
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(vv)=self.m.get(&key){
            let p = vv.range(..=timestamp);
            if let Some(i)=p.last(){
               return i.1.clone();
            }
        }
        String::new()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
// @lc code=end

