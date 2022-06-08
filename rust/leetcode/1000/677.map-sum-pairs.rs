/*
 * @lc app=leetcode id=677 lang=rust
 *
 * [677] Map Sum Pairs
 */

// @lc code=start
struct MapSum {
m:std::collections::HashMap<String,i32>,
pm:std::collections::HashMap<String,i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    fn new() -> Self {
        Self{m:std::collections::HashMap::new(),
        pm:std::collections::HashMap::new(),}
    }
    
    fn insert(&mut self, key: String, val: i32) {
        let mut delta = val;
        if let Some(v)=self.m.get(&key){
            delta-=v;
        }
        *self.m.entry(key.clone()).or_insert(val)=val;
        let ss = key.chars().collect::<Vec<char>>();
        for i in 1..=ss.len(){
           *self.pm.entry(ss[0..i].iter().collect::<String>()).or_insert(0)+=delta;
        }
    }
    
    fn sum(&self, prefix: String) -> i32 {
        *self.pm.get(&prefix).unwrap_or(&0)
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
// @lc code=end

