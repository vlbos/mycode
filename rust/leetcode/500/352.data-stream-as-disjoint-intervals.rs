/*
 * @lc app=leetcode id=352 lang=rust
 *
 * [352] Data Stream as Disjoint Intervals
 */

// @lc code=start
use std::collections::BTreeMap;
struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, val: i32) {
        let mut left_side = false;
        let l0 = self.intervals.range(..=val).rev().next();
        if let Some((&k, &v)) = l0 {
            if val >= k && val <= v {
                return;
            }
            left_side = v + 1 == val;
        }
        let mut right_side = false;
        let l1 = self.intervals.range(val + 1..).next();
        if let Some((&k, &v)) = l1 {
            right_side = k - 1 == val;
        }
        if left_side && right_side {
            let (left, right) = (*l0.unwrap().0, *l1.unwrap().1);
            let (k0, k1) = (*l0.unwrap().0, *l1.unwrap().0);
            self.intervals.remove(&k0);
            self.intervals.remove(&k1);
            self.intervals.insert(left, right);
        } else if left_side {
            let left = *l0.unwrap().0;
            self.intervals.entry(left).and_modify(|x| *x += 1);
        } else if right_side {
            let right = *l1.unwrap().1;
            let k1 = *l1.unwrap().0;
            self.intervals.remove(&k1);
            self.intervals.insert(k1 - 1, right);
        } else {
            self.intervals.insert(val, val);
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
       self.intervals
            .iter()
            .map(|x| vec![*x.0, *x.1])
            .collect::<Vec<Vec<i32>>>()
    }
}
/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */
// @lc code=end
