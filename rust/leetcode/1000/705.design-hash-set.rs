/*
 * @lc app=leetcode id=705 lang=rust
 *
 * [705] Design HashSet
 */

// @lc code=start
struct MyHashSet {
v:Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{v:[0;1000001].to_vec()}
    }
    
    fn add(&mut self, key: i32) {
        self.v[key as usize]=1;
    }
    
    fn remove(&mut self, key: i32) {
        self.v[key  as usize]=0;
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.v[key  as usize]!=0
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
// @lc code=end

