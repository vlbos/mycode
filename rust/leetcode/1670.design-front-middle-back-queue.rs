/*
 * @lc app=leetcode id=1670 lang=rust
 *
 * [1670] Design Front Middle Back Queue
 */

// @lc code=start
use std::collections::VecDeque;
struct FrontMiddleBackQueue {
    ql: VecDeque<i32>,
    qr: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            ql: VecDeque::new(),
            qr: VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.ql.push_front(val);
        self.update();
    }

    fn push_middle(&mut self, val: i32) {
        if self.ql.len() == self.qr.len() {
            self.qr.push_front(val);
        } else {
            self.ql.push_back(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.qr.push_back(val);
        self.update();
    }

    fn pop_front(&mut self) -> i32 {
        let mut ans = -1;
        if let Some(i) = self.ql.pop_front() {
            ans = i;
        } else if let Some(i) = self.qr.pop_front() {
            ans = i;
        }
        self.update();
        ans
    }

    fn pop_middle(&mut self) -> i32 {
        let mut ans = -1;
        if self.ql.len() == self.qr.len() {
            if let Some(i) = self.ql.pop_back() {
                ans = i;
            }
        } else if let Some(i) = self.qr.pop_front() {
            ans = i;
        }
        ans
    }

    fn pop_back(&mut self) -> i32 {
        let mut ans = -1;
        if let Some(i) = self.qr.pop_back() {
            ans = i;
        } else if let Some(i) = self.ql.pop_back() {
            ans = i;
        }
        self.update();
        ans
    }
    fn update(&mut self) {
        if self.ql.len() > self.qr.len() {
            if let Some(v) = self.ql.pop_back() {
                self.qr.push_front(v);
            }
        } else if self.ql.len() + 2 == self.qr.len() {
            if let Some(v) = self.qr.pop_front() {
                self.ql.push_back(v);
            }
        }
    }
}
/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
// @lc code=end
