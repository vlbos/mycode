/*
 * @lc app=leetcode id=1206 lang=rust
 *
 * [1206] Design Skiplist
 */

// @lc code=start
const max_level: i32 = 32;
const p_factor: f64 = 0.25;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug)]
struct Node {
    value: i32,
    next: Vec<Option<Rc<RefCell<Node>>>>,
}
impl Node {
    fn new(value: i32, len: usize) -> Self {
        Self {
            value,
            next: vec![None; len],
        }
    }
}
struct Skiplist {
    head: Option<Rc<RefCell<Node>>>,
    current_level: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {
    fn new() -> Self {
        Self {
            head: Some(Rc::new(RefCell::new(Node::new(-1, max_level as usize)))),
            current_level: 1,
        }
    }

    fn find_closest(
        &self,
        node: &Option<Rc<RefCell<Node>>>,
        level_index: usize,
        value: i32,
    ) -> Option<Rc<RefCell<Node>>> {
        let mut node = node.clone();
        while node.as_ref().unwrap().borrow().next[level_index].is_some()
            && value
                > node.as_ref().unwrap().borrow().next[level_index]
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .value
        {
            let nnode = node.as_ref().unwrap().borrow().next[level_index].clone();
            node = nnode;
        }
        node.clone()
    }
    fn random_level(&self) -> i32 {
        let mut level = 1;
        use rand::Rng;
        let mut rng = rand::thread_rng();
        while rng.gen::<f64>() < p_factor && level < max_level {
            level += 1;
        }
        level
    }
    fn search(&self, target: i32) -> bool {
        let mut node = self.head.clone();
        for i in (0..self.current_level).rev() {
            node = self.find_closest(&node, i, target);
            if node.as_ref().unwrap().borrow().next[i].is_some()
                && target
                    == node.as_ref().unwrap().borrow().next[i]
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .value
            {
                return true;
            }
        }
        false
    }

    fn add(&mut self, num: i32) {
        let level = self.random_level() as usize;
        let mut node = self.head.clone();
        let mut new_node = Node::new(num, level);
        for i in (self.current_level..level).rev() {
            node = self.find_closest(&node, i, num);
        }
        for i in (0..self.current_level).rev() {
            node = self.find_closest(&node, i, num);
            if i < level {
                if node.as_ref().unwrap().borrow().next[i].is_none() {
                    node.as_mut().unwrap().borrow_mut().next[i] =
                        Some(Rc::new(RefCell::new(new_node.clone())));
                } else {
                    new_node.next[i] = node.as_mut().unwrap().borrow_mut().next[i].take();
                    node.as_mut().unwrap().borrow_mut().next[i] =
                        Some(Rc::new(RefCell::new(new_node.clone())));
                }
            }
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        let mut ans = false;
        let mut node = self.head.clone();
        for i in (0..self.current_level).rev() {
            node = self.find_closest(&node, i, num);
            if node.as_ref().unwrap().borrow().next[i].is_some()
                && num
                    == node.as_ref().unwrap().borrow().next[i]
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .value
            {
                let s = node.as_ref().unwrap().borrow().next[i]
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .next[i]
                    .clone();
                node.as_mut().unwrap().borrow_mut().next[i] = s;
                ans = true;
            }
        }
        ans
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
// @lc code=end
