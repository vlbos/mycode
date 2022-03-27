/*
 * @lc app=leetcode id=1146 lang=rust
 *
 * [1146] Snapshot Array
 */

// @lc code=start
struct SnapshotArray {
    m:std::collections::HashMap<i32,std::collections::BTreeMap<i32,i32>>,
    snap_id:i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
       Self{m:std::collections::HashMap::new(),snap_id:0}
    }
    
    fn set(&mut self, index: i32, val: i32) {
        *self.m.entry(index).or_insert(std::collections::BTreeMap::new()).entry(self.snap_id).or_insert(0)=val;
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_id+=1;
        self.snap_id-1
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        if let Some(s)=self.m.get(&index){
           return  *s.range(..=snap_id).last().unwrap_or((&0,&0)).1;
        }
        0
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
// @lc code=end

