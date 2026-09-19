// [3369\. Design an Array Statistics Tracker 🔒](https://leetcode.com/problems/design-an-array-statistics-tracker)
// ================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Design a data structure that keeps track of the values in it and answers some queries regarding their mean, median, and mode.

// Implement the `StatisticsTracker` class.

// *   `StatisticsTracker()`: Initialize the `StatisticsTracker` object with an empty array.
// *   `void addNumber(int number)`: Add `number` to the data structure.
// *   `void removeFirstAddedNumber()`: Remove the earliest added number from the data structure.
// *   `int getMean()`: Return the floored **mean** of the numbers in the data structure.
// *   `int getMedian()`: Return the **median** of the numbers in the data structure.
// *   `int getMode()`: Return the **mode** of the numbers in the data structure. If there are multiple modes, return the smallest one.

// **Note**:

// *   The **mean** of an array is the sum of all the values divided by the number of values in the array.
// *   The **median** of an array is the middle element of the array when it is sorted in non-decreasing order. If there are two choices for a median, the larger of the two values is taken.
// *   The **mode** of an array is the element that appears most often in the array.

// **Example 1:**

// **Input:**
// \["StatisticsTracker", "addNumber", "addNumber", "addNumber", "addNumber", "getMean", "getMedian", "getMode", "removeFirstAddedNumber", "getMode"\]
// \[\[\], \[4\], \[4\], \[2\], \[3\], \[\], \[\], \[\], \[\], \[\]\]

// **Output:**
// \[null, null, null, null, null, 3, 4, 4, null, 2\]

// **Explanation**

// StatisticsTracker statisticsTracker = new StatisticsTracker();
// statisticsTracker.addNumber(4); // The data structure now contains \[4\]
// statisticsTracker.addNumber(4); // The data structure now contains \[4, 4\]
// statisticsTracker.addNumber(2); // The data structure now contains \[4, 4, 2\]
// statisticsTracker.addNumber(3); // The data structure now contains \[4, 4, 2, 3\]
// statisticsTracker.getMean(); // return 3
// statisticsTracker.getMedian(); // return 4
// statisticsTracker.getMode(); // return 4
// statisticsTracker.removeFirstAddedNumber(); // The data structure now contains \[4, 2, 3\]
// statisticsTracker.getMode(); // return 2

// **Example 2:**

// **Input:**
// \["StatisticsTracker", "addNumber", "addNumber", "getMean", "removeFirstAddedNumber", "addNumber", "addNumber", "removeFirstAddedNumber", "getMedian", "addNumber", "getMode"\]
// \[\[\], \[9\], \[5\], \[\], \[\], \[5\], \[6\], \[\], \[\], \[8\], \[\]\]

// **Output:**
// \[null, null, null, 7, null, null, null, null, 6, null, 5\]

// **Explanation**

// StatisticsTracker statisticsTracker = new StatisticsTracker();
// statisticsTracker.addNumber(9); // The data structure now contains \[9\]
// statisticsTracker.addNumber(5); // The data structure now contains \[9, 5\]
// statisticsTracker.getMean(); // return 7
// statisticsTracker.removeFirstAddedNumber(); // The data structure now contains \[5\]
// statisticsTracker.addNumber(5); // The data structure now contains \[5, 5\]
// statisticsTracker.addNumber(6); // The data structure now contains \[5, 5, 6\]
// statisticsTracker.removeFirstAddedNumber(); // The data structure now contains \[5, 6\]
// statisticsTracker.getMedian(); // return 6
// statisticsTracker.addNumber(8); // The data structure now contains \[5, 6, 8\]
// statisticsTracker.getMode(); // return 5

// **Constraints:**

// *   `1 <= number <= 109`
// *   At most, `105` calls will be made to `addNumber`, `removeFirstAddedNumber`, `getMean`, `getMedian`, and `getMode` in total.
// *   `removeFirstAddedNumber`, `getMean`, `getMedian`, and `getMode` will be called only if there is at least one element in the data structure.
use std::collections::{BTreeSet, BinaryHeap, HashMap, VecDeque};
#[allow(dead_code)]
pub struct StatisticsTracker {
    q: VecDeque<i32>,
    l: BinaryHeap<i32>,
    r: BinaryHeap<i32>,
    cnt: HashMap<i32, i32>,
    delayed: HashMap<i32, i32>,
    seq: BTreeSet<(i32, i32)>,
    l_len: i32,
    r_len: i32,
    s: i64,
}
#[allow(dead_code)]
impl StatisticsTracker {
    fn new() -> Self {
        Self {
            q: VecDeque::new(),
            l: BinaryHeap::new(),
            r: BinaryHeap::new(),
            cnt: HashMap::new(),
            delayed: HashMap::new(),
            seq: BTreeSet::new(),
            l_len: 0,
            r_len: 0,
            s: 0,
        }
    }

    // 清理堆延迟删除标记
    fn prune(&mut self, signed: i32) {
        let bh = if signed > 0 { &mut self.l } else { &mut self.r };
        while let Some(&top) = bh.peek() {
            let real_val = top * signed;
            if let Some(del) = self.delayed.get_mut(&real_val) {
                *del -= 1;
                if *del == 0 {
                    self.delayed.remove(&real_val);
                }
                bh.pop();
            } else {
                break;
            }
        }
    }

    // 平衡：l_len == r_len 或 l_len = r_len + 1
    fn rebalance(&mut self) {
        self.prune(1);
        self.prune(-1);
        if self.l_len > self.r_len + 1 {
            let val = self.l.pop().unwrap();
            self.r.push(-val);
            self.l_len -= 1;
            self.r_len += 1;
        } else if self.l_len < self.r_len {
            let val = -self.r.pop().unwrap();
            self.l.push(val);
            self.l_len += 1;
            self.r_len -= 1;
        }
    }

    fn add_number(&mut self, number: i32) {
        self.s += number as i64;
        self.q.push_back(number);

        // 更新计数与众数有序集合
        let old_cnt = *self.cnt.get(&number).unwrap_or(&0);
        if old_cnt > 0 {
            self.seq.remove(&(-old_cnt, number));
        }
        let new_cnt = old_cnt + 1;
        *self.cnt.entry(number).or_insert(0) = new_cnt;
        self.seq.insert((-new_cnt, number));

        // 加入对应堆
        self.prune(1);
        self.prune(-1);
        if self.l.is_empty() || *self.l.peek().unwrap() >= number {
            self.l.push(number);
            self.l_len += 1;
        } else {
            self.r.push(-number);
            self.r_len += 1;
        }
        self.rebalance();
    }

    fn remove_first_added_number(&mut self) {
        let number = self.q.pop_front().unwrap();
        self.s -= number as i64;

        // 更新计数、众数集合
        let old_cnt = self.cnt[&number];
        self.seq.remove(&(-old_cnt, number));
        let new_cnt = old_cnt - 1;
        if new_cnt == 0 {
            self.cnt.remove(&number);
        } else {
            *self.cnt.get_mut(&number).unwrap() = new_cnt;
            self.seq.insert((-new_cnt, number));
        }

        // 标记延迟删除
        self.prune(1);
        if !self.l.is_empty() && *self.l.peek().unwrap() >= number {
            *self.delayed.entry(number).or_insert(0) += 1;
            self.l_len -= 1;
        } else {
            *self.delayed.entry(number).or_insert(0) += 1;
            self.r_len -= 1;
        }
        self.rebalance();
    }

    fn get_mean(&self) -> i32 {
        (self.s / self.q.len() as i64) as i32
    }

    // 严格还原题目判题中位数规则：偶数取右堆最小值
    fn get_median(&mut self) -> i32 {
        self.prune(1);
        self.prune(-1);
        if self.l_len == self.r_len {
            // 偶数个，取右堆最小
            -self.r.peek().unwrap()
        } else {
            // 奇数个，取左堆最大
            *self.l.peek().unwrap()
        }
    }

    fn get_mode(&self) -> i32 {
        self.seq.first().unwrap().1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_statistics_tracker_1() {
        let mut statistics_tracker = StatisticsTracker::new();
        statistics_tracker.add_number(4);
        statistics_tracker.add_number(4);
        statistics_tracker.add_number(2);
        statistics_tracker.add_number(3);
        assert_eq!(3, statistics_tracker.get_mean());
        assert_eq!(4, statistics_tracker.get_median());
        assert_eq!(4, statistics_tracker.get_mode());
        statistics_tracker.remove_first_added_number();
        assert_eq!(2, statistics_tracker.get_mode());
    }
    #[test]
    pub fn test_statistics_tracker_2() {
        let mut statistics_tracker = StatisticsTracker::new();
        statistics_tracker.add_number(9);
        statistics_tracker.add_number(5);
        assert_eq!(7, statistics_tracker.get_mean());
        statistics_tracker.remove_first_added_number();
        statistics_tracker.add_number(5);
        statistics_tracker.add_number(6);
        statistics_tracker.remove_first_added_number();
        assert_eq!(6, statistics_tracker.get_median());
        statistics_tracker.add_number(8);
        assert_eq!(5, statistics_tracker.get_mode());
    }
}
