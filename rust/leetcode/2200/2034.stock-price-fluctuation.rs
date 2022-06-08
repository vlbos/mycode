/*
 * @lc app=leetcode id=2034 lang=rust
 *
 * [2034] Stock Price Fluctuation
 */

// @lc code=start
use std::collections::BTreeMap;
use std::collections::HashMap;
struct StockPrice {
    sp: HashMap<i32, i32>,
    p: BTreeMap<i32, i32>,
    ts: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            sp: HashMap::new(),
            p: BTreeMap::new(),
            ts: 0,
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.ts = self.ts.max(timestamp);
        if let Some(mut v) = self.sp.get_mut(&timestamp) {
            let n = *self.p.get(v).unwrap_or(&0);
            if n == 1 {
                self.p.remove(v);
            } else {
                self.p.entry(*v).and_modify(|x| *x -= 1);
            }
            *v = price;
        } else {
            self.sp.insert(timestamp, price);
        }
        *self.p.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        *self.sp.get(&self.ts).unwrap_or(&0)
    }

    fn maximum(&self) -> i32 {
        *self.p.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.p.iter().next().unwrap().0
    }
}
/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
// @lc code=end
