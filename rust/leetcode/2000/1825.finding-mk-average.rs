/*
 * @lc app=leetcode id=1825 lang=rust
 *
 * [1825] Finding MK Average
 */

// @lc code=start
use std::collections::BTreeMap;
use std::collections::VecDeque;
struct MKAverage {
    m: i32,
    k: i32,
    sum: i32,
    nums: VecDeque<i32>,
    lower: BTreeMap<i32, i32>,
    middle: BTreeMap<i32, i32>,
    upper: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            m,
            k,
            sum: 0,
            nums: VecDeque::new(),
            lower: BTreeMap::new(),
            middle: BTreeMap::new(),
            upper: BTreeMap::new(),
        }
    }

    fn add_element(&mut self, num: i32) {
       self.nums.push_back(num);
        if !self.lower.is_empty() && *self.lower.iter().next_back().unwrap().0 >= num {
            self.lower.entry(num).and_modify(|x| *x += 1).or_insert(1);
        } else if !self.upper.is_empty() && *self.upper.iter().next().unwrap().0 <= num {
            self.upper.entry(num).and_modify(|x| *x += 1).or_insert(1);
        } else {
            self.middle.entry(num).and_modify(|x| *x += 1).or_insert(1);
            self.sum += num;
        }
        let k = self.k ;
        let shift_left = |l: &mut BTreeMap<i32, i32>, r: &mut BTreeMap<i32, i32>| {
            let (&k, &v) = r.iter().next().unwrap();
            l.entry(k).and_modify(|x| *x += 1).or_insert(1);
            if v == 1 {
                r.remove(&k);
            } else {
                r.entry(k).and_modify(|x| *x -= 1);
            }
        };
        let shift_right = |l: &mut BTreeMap<i32, i32>, r: &mut BTreeMap<i32, i32>| {
            let (&k, &v) = l.iter().next_back().unwrap();
            r.entry(k).and_modify(|x| *x += 1).or_insert(1);
            if v == 1 {
                l.remove(&k);
            } else {
                l.entry(k).and_modify(|x| *x -= 1);
            }
        };
        while self.lower.values().sum::<i32>() > k {
            self.sum += *self.lower.iter().next_back().unwrap().0;
            shift_right(&mut self.lower, &mut self.middle);
        }
        while self.upper.values().sum::<i32>()  > k {
            self.sum += *self.upper.iter().next().unwrap().0;
            shift_left(&mut self.middle, &mut self.upper);
        }
        let m = self.m as usize;
        if self.nums.len() > m {
            let d = self.nums.pop_front().unwrap();
            if let Some(v) = self.lower.remove(&d) {
                if v > 1 {
                    self.lower.insert(d, v - 1);
                }
            } else if let Some(v) = self.middle.remove(&d) {
                if v > 1 {
                    self.middle.insert(d, v - 1);
                }
                self.sum -= d;
            } else if let Some(v) = self.upper.remove(&d) {
                if v > 1 {
                    self.upper.insert(d, v - 1);
                }
            }
        }
        if self.nums.len() >= m {
            while self.lower.values().sum::<i32>() < k {
                self.sum -= *self.middle.iter().next().unwrap().0;
                shift_left(&mut self.lower, &mut self.middle);
            }
            while self.upper.values().sum::<i32>() < k {
                self.sum -= *self.middle.iter().next_back().unwrap().0;
                shift_right(&mut self.middle, &mut self.upper);
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.nums.len() < self.m as usize {
            return -1;
        }
        self.sum / (self.m - self.k * 2)
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */
// @lc code=end
