// 346\. Moving Average from Data Stream
// =====================================

// Given a stream of integers and a window size, calculate the moving average of all integers in the sliding window.

// **Example:**

// MovingAverage m = new MovingAverage(3);
// m.next(1) = 1
// m.next(10) = (1 + 10) / 2
// m.next(3) = (1 + 10 + 3) / 3
// m.next(5) = (10 + 3 + 5) / 3

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Indeed](https://leetcode.ca/tags/#Indeed) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Twitter](https://leetcode.ca/tags/#Twitter) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
use std::collections::VecDeque;

pub struct MovingAverage {
    window: VecDeque<i32>,
    sum: i32,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    pub fn new(size: i32) -> Self {
        MovingAverage {
            window: VecDeque::new(),
            sum: 0,
            size: size as usize,
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        if self.window.len() == self.size {
            if let Some(first) = self.window.pop_front() {
                self.sum -= first;
            } else {
                return 0 as f64;
            }
        }
        self.window.push_back(val);
        self.sum += val;
        (self.sum as f64) / (self.window.len() as f64)
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::assert_feq;

    #[test]
    pub fn test_moveing_average_1() {
        let mut m = MovingAverage::new(3);
        assert_feq(m.next(1), 1f64);
        assert_feq(m.next(10), ((1 + 10) as f64) / 2f64);
        assert_feq(m.next(3), ((1 + 10 + 3) as f64) / 3f64);
        assert_feq(m.next(5), ((10 + 3 + 5) as f64) / 3f64);
    }

    #[test]
    pub fn test_moveing_average_2() {
        let mut m = MovingAverage::new(0);
        assert_feq(m.next(1), 0f64);
        assert_feq(m.next(10), 0f64);
        assert_feq(m.next(3), 0f64);
        assert_feq(m.next(5), 0f64);
    }
}
