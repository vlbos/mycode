/*
 * @lc app=leetcode id=460 lang=rust
 *
 * [460] LFU Cache
 */

// @lc code=start
use std::collections::BTreeSet;
use std::collections::HashMap;
struct LFUCache {
    capacity: usize,
    time: i32,
    key_table: HashMap<i32, Vec<i32>>,
    s: BTreeSet<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            time: 0,
            key_table: HashMap::new(),
            s: BTreeSet::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 || self.key_table.get(&key).is_none() {
            return -1;
        }
        let val = self.key_table.get(&key).unwrap().clone();
        let mut newval = val.clone();
        newval[0] += 1;
        newval[1] = self.time;
        self.time += 1;
        self.s.remove(&val);
        self.s.insert(newval.clone());
        self.key_table
            .entry(key)
            .and_modify(|x| *x = newval.clone());
        val[3]
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        let is_insert = self.key_table.get(&key).is_none();
        if is_insert {
            if self.s.len() == self.capacity {
                let val = self.s.iter().next().unwrap().clone();
                self.key_table.remove(&val[2]);
                self.s.remove(&val);
            }
            let newval = vec![1, self.time, key, value];
            self.key_table.insert(key, newval.clone());
            self.s.insert(newval.clone());
        } else {
            let val = self.key_table.get(&key).unwrap().clone();
            let mut newval = val.clone();
            newval[0] += 1;
            newval[1] = self.time;
            newval[3] = value;
            self.s.remove(&val);
            self.s.insert(newval.clone());
            self.key_table
                .entry(key)
                .and_modify(|x| *x = newval.clone());
        }

        self.time += 1;
    }
}
/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end
