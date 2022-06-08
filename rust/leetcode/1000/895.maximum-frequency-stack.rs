/*
 * @lc app=leetcode id=895 lang=rust
 *
 * [895] Maximum Frequency Stack
 */

// @lc code=start
use std::collections::HashMap;
struct FreqStack {
    freq: HashMap<i32, i32>,
    group: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            group: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        *self.freq.entry(val).or_insert(0) += 1;
        let freq = *self.freq.get(&val).unwrap();
        if *self.freq.get(&val).unwrap() > self.max_freq {
            self.max_freq = *self.freq.get(&val).unwrap();
        }
        self.group.entry(freq).or_insert(Vec::new()).push(val);
    }

    fn pop(&mut self) -> i32 {
        let x = self.group.get_mut(&self.max_freq).unwrap().pop().unwrap();
        self.freq.entry(x).and_modify(|f|*f-=1);
        if self.group.get_mut(&self.max_freq).unwrap().is_empty() {
            self.max_freq -= 1;
        }
        x
    }
}

    /**
     * Your FreqStack object will be instantiated and called as such:
     * let obj = FreqStack::new();
     * obj.push(val);
     * let ret_2: i32 = obj.pop();
     */
// @lc code=end
