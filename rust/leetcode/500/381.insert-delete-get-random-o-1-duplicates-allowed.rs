/*
 * @lc app=leetcode id=381 lang=rust
 *
 * [381] Insert Delete GetRandom O(1) - Duplicates allowed
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
struct RandomizedCollection {
    idx: HashMap<i32, HashSet<usize>>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            idx: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.idx
            .entry(val)
            .or_insert(HashSet::new())
            .insert(self.nums.len());
        self.nums.push(val);
        self.idx.get(&val).unwrap().len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.idx.get(&val).is_none() || self.idx.get(&val).unwrap().is_empty() {
            return false;
        }
        let i = *self.idx.get(&val).unwrap().iter().next().unwrap();
        self.idx.get_mut(&val).unwrap().remove(&i);
        let n = self.nums.len();
        if n > 1 && i != (n - 1) {
            self.nums.swap(i, n - 1);
            self.idx.get_mut(&self.nums[i]).unwrap().remove(&(n - 1));
            self.idx.get_mut(&self.nums[i]).unwrap().insert(i);
        }
        self.nums.pop();

        if self.idx.get(&val).unwrap().is_empty() {
            self.idx.remove(&val);
        }
        true
    }

    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let n = self.nums.len();
        let n1: usize = rng.gen::<usize>();

        self.nums[n1 % n]
    }
}
/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end
