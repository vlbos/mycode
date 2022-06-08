/*
 * @lc app=leetcode id=850 lang=rust
 *
 * [850] Rectangle Area II
 */

// @lc code=start
struct Node {
    start: i32,
    end: i32,
    x: Vec<i32>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    count: i32,
    total: i64,
}
impl Node {
    fn new(start: i32, end: i32, x: Vec<i32>) -> Self {
        Self {
            start,
            end,
            x,
            left: None,
            right: None,
            count: 0,
            total: 0,
        }
    }
    fn get_range_mid(&self) -> i32 {
        self.start + (self.end - self.start) / 2
    }
    fn update(&mut self, i: i32, j: i32, val: i32) -> i64 {
        if i >= j {
            return self.total;
        }
        if self.start == i && self.end == j {
            self.count += val;
        } else {
            let mid = self.get_range_mid();
            if let Some( left) = &mut self.left {
                left.as_mut().update(i, j.min(mid), val);
            } else {
                let mut left = Node::new(self.start, self.get_range_mid(), self.x.clone());
                left.update(i, j.min(self.get_range_mid()), val);
                self.left = Some(Box::new(left));
            }
            if let Some( right) = &mut self.right {
                right.update(i.max(mid), j, val);
            } else {
                let mut right = Node::new(self.get_range_mid(), self.end, self.x.clone());
                right.update(i.max(self.get_range_mid()), j, val);
                self.right = Some(Box::new(right));
            }
        }
        if self.count > 0 {
            self.total = (self.x[self.end as usize] - self.x[self.start as usize]) as i64;
        } else {
            self.total = if self.left.is_none(){0}else{self.left.as_ref().unwrap().total} + if self.right.is_none() {0}else{self.right.as_ref().unwrap().total};
        }
        self.total
    }
}
impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let (mut open, mut close) = (1, -1);
        let n = rectangles.len();
        let mut events = Vec::new();
        let mut x = Vec::new();
        for rect in &rectangles {
            events.push(vec![rect[1], open, rect[0], rect[2]]);
            events.push(vec![rect[3], close, rect[0], rect[2]]);
            x.push(rect[0]);
            x.push(rect[2]);
        }
        events.sort();
        x.sort();
        x.dedup();
        let xi: std::collections::HashMap<i32, i32> = x.clone()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32))
            .collect();
        let mut active = Node::new(0, x.len() as i32 - 1, x.clone());
        let mut ans = 0;
        let mut cur_x_sum = 0;
        let mut cur_y = events[0][0];
        for event in &events {
            let (y, typ, x1, x2) = (event[0], event[1], event[2], event[3]);
            ans += cur_x_sum * (y - cur_y) as i64;
            
            cur_x_sum = active.update(*xi.get(&x1).unwrap(), *xi.get(&x2).unwrap(), typ);
            cur_y = y;
        }
        (ans % 1_000_000_007) as _
    }
}
// @lc code=end
