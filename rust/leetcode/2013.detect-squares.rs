/*
 * @lc app=leetcode id=2013 lang=rust
 *
 * [2013] Detect Squares
 */

// @lc code=start
use std::collections::HashMap;
struct DetectSquares {
    m: HashMap<i32, HashMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self
            .m
            .entry(point[1])
            .or_insert(HashMap::new())
            .entry(point[0])
            .or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (x, y) = (point[0], point[1]);
        if !self.m.contains_key(&y) {
            return 0;
        }
        let y_cnt = &self.m[&y];
        for (&col, col_cnt) in &self.m {
            if col != y {
                let d = col - y;
                ans += (*col_cnt.get(&x).unwrap_or(&0))
                    * (*y_cnt.get(&(x + d)).unwrap_or(&0))
                    * (*col_cnt.get(&(x + d)).unwrap_or(&0));
                ans += (*col_cnt.get(&x).unwrap_or(&0))
                    * (*y_cnt.get(&(x - d)).unwrap_or(&0))
                    * (*col_cnt.get(&(x - d)).unwrap_or(&0));
            }
        }
        ans
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
// @lc code=end
