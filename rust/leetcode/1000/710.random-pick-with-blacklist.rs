/*
 * @lc app=leetcode id=710 lang=rust
 *
 * [710] Random Pick with Blacklist
 */

// @lc code=start
use std::collections::HashMap;
struct Solution {
    m: HashMap<i32, i32>,
    wlen: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        use std::collections::HashSet;
        let wlen = n - blacklist.len() as i32;
        let mut w = (wlen..n).collect::<HashSet<i32>>();
        w = w
            .difference(&blacklist.iter().cloned().collect::<HashSet<i32>>()).cloned()
            .collect();
        let mut m = HashMap::new();
        let mut it = w.iter();
        for &x in &blacklist {
            if x < wlen {
                m.insert(x, *it.next().unwrap());
            }
        }
        Self { m, wlen }
    }

    fn pick(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let k = rng.gen_range(0,self.wlen) ;
        *self.m.get(&k).unwrap_or(&k)
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */
// @lc code=end
