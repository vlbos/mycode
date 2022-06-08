/*
 * @lc app=leetcode id=1993 lang=rust
 *
 * [1993] Operations on Tree
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
struct LockingTree {
    locked: HashMap<i32, i32>,
    child: HashMap<i32, HashSet<i32>>,
    p: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut child = HashMap::new();
        let mut locked= HashMap::new();
        for (i, &n) in parent.iter().enumerate() {
            child.entry(n).or_insert(HashSet::new()).insert(i as i32);
        }
        Self {
            p: parent,
            child,
            locked,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locked.contains_key(&num) {
            return false;
        }
        self.locked.insert(num, user);
        true
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.locked.contains_key(&num)
            &&*self.locked.get(&num).unwrap_or(&(self.p.len() as i32)) == user
        {
            self.locked.remove(&num);
            return true;
        }
        false
    }
    fn dfs(&self, mut num: i32) -> bool {
        while num != -1 {
            if self.locked.contains_key(&num) {
                return false;
            }
            num = self.p[num as usize];
        }
        true
    }
    fn ddfs(&self, num: i32, f: i32) -> bool {
        if self.locked.contains_key(&num) && num != f {
            return true;
        }
        let mut ans = false;
        if let Some(c) = self.child.get(&num) {
            for &i in c {
                ans |= self.ddfs(i, f);
            }
        }
        ans
    }
    fn dddfs(&mut self, num: i32) {
        self.locked.remove(&num);
        let mut cc = HashSet::new();
         if let Some(c) =  self.child.get(&num){
             cc=c.clone();
         }
            for i in cc {
                self.dddfs(i);
            }
    }
    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.dfs(num) && self.ddfs(num, num) {
            self.dddfs(num);
            self.locked.insert(num, user);
            return true;
        }
        false
    }
}
/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */
// @lc code=end
