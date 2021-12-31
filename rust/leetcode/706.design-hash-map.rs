/*
 * @lc app=leetcode id=706 lang=rust
 *
 * [706] Design HashMap
 */

// @lc code=start
struct MyHashMap {
 v:Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{v:[-1;1000001].to_vec()}
    }
    
    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.v[key as usize]=value;
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
            self.v[key as usize]
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.v[key as usize]=-1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
// @lc code=end

