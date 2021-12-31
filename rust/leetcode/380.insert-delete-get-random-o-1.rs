/*
 * @lc app=leetcode id=380 lang=rust
 *
 * [380] Insert Delete GetRandom O(1)
 */

// @lc code=start
struct RandomizedSet {
      e:Vec<i32>,
      h:std::collections::HashMap<i32,usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self{e:Vec::new(),h:std::collections::HashMap::new()}
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.h.contains_key(&val){
            return false;
        }
        self.e.push(val);
        self.h.insert(val,self.e.len()-1);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if !self.h.contains_key(&val){
            return false;
        }
        let mut ni=0;
        if let Some(i)=self.h.get(&val){
             ni=*i;
             self.e[ni]=*self.e.last().unwrap();
             self.h.remove(&val);
        }
        if let Some(j)=self.h.get_mut(&self.e[ni]){
            *j=ni;
        }
        self.e.pop();

        true
    }
    
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0,self.e.len());
        self.e[i]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end

