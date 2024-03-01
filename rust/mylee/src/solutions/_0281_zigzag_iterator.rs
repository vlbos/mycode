// 281\. Zigzag Iterator
// =====================

// Given two 1d vectors, implement an iterator to return their elements alternately.

// **Example:**

// **Input:**
// v1 = \[1,2\]
// v2 = \[3,4,5,6\]

// **Output:** `[1,3,2,4,5,6]

// **Explanation:**` By calling _next_ repeatedly until _hasNext_ returns `false`,
//              the order of elements returned by _next_ should be: `[1,3,2,4,5,6]`.

// **Follow up**: What if you are given `k` 1d vectors? How well can your code be extended to such cases?

// **Clarification** **for the follow up question****:**
// The "Zigzag" order is not clearly defined and is ambiguous for `k > 2` cases. If "Zigzag" does not look right to you, replace "Zigzag" with "Cyclic". For example:

// **Input:**
// \[1,2,3\]
// \[4,5,6,7\]
// \[8,9\]

// **Output:** `[1,4,8,2,5,9,3,6,7]`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Cruise Automation](https://leetcode.ca/tags/#Cruise%20Automation) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
// @lc code=start
use std::collections::VecDeque;
pub struct ZigzagIterator {
    // vs: Vec<Vec<i32>>,
    // i: usize,
    // j: usize,
    // ei: usize,
    pub q: VecDeque<i32>,
}

impl ZigzagIterator {
    /** initialize your data structure here. */

    pub fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        // let vs = vec![v1, v2];
        // let mut end_line = 0;
        // let mut line_max = usize::min_value();
        // for (i, v) in vs.iter().enumerate() {
        //     if v.len() >= line_max {
        //         line_max = v.len();
        //         end_line = i;
        //     }
        // }
        // ZigzagIterator {
        //     vs,
        //     i: 0,
        //     j: 0,
        //     ei: end_line,
        // }
        let mut i = 0;
        let mut q = VecDeque::new();
        while i / 2 < v1.len() && i / 2 < v2.len() {
            q.push_back(if i % 2 == 0 { v1[i / 2] } else { v2[i / 2] });
            i += 1;
        }
        if i / 2 < v1.len() {
            q.extend(&v1[i / 2..]);
        }
        if i / 2 < v2.len() {
            q.extend(&v2[i / 2..]);
        }
        Self { q }
    }

    pub fn next(&mut self) -> i32 {
        // if !self.has_next(){
        // return -1;
        // }
        // // self.has_next();
        // // let res = self.vs[self.i][self.j];
        // // self.i += 1;
        // // res
        // let ans = self.zz[self.k][self.zi[self.k]];
        // self.zi[self.k] += 1;
        // self.k += 1;
        // if self.zi[self.k] == self.zz[self.k].len() {
        //     self.k += 1;
        // }
        // ans
        self.q.pop_front().unwrap()
    }

    pub fn has_next(&self) -> bool {
        // let line_max = self.vs[self.ei].len();
        // loop {
        //     if (self.i > self.ei && self.j + 1 == line_max) || self.j >= line_max {
        //         return false;
        //     } else if self.i < self.vs.len() {
        //         if self.j < self.vs[self.i].len() {
        //             return true;
        //         } else {
        //             self.i += 1;
        //         }
        //     }
        //     if self.i >= self.vs.len() {
        //         self.i %= self.vs.len();
        //         self.j += 1;
        //     }
        // }
        // self.zi.iter().zip(&self.zz).any(|(i, b)| *i < b.len())
        !self.q.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_zigzag_iterator1() {
        let mut zz = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
        let mut res = vec![];
        while zz.has_next() {
            res.push(zz.next());
        }
        assert_eq!(res, vec![1, 3, 2, 4, 5, 6]);
    }

    #[test]
    pub fn test_zigzag_iterator2() {
        let mut zz = ZigzagIterator::new(vec![], vec![]);
        let mut res = vec![];
        while zz.has_next() {
            res.push(zz.next());
        }
        assert_eq!(res, vec![]);
    }
}
