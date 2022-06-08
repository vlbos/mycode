/*
 * @lc app=leetcode id=432 lang=rust
 *
 * [432] All O`one Data Structure
 */

// @lc code=start
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

type Node<K, V> = Rc<RefCell<LinkedListNode<K, V>>>;
type Link<K, V> = Option<Node<K, V>>;

#[derive(Debug)]
struct LinkedListNode<K, V> {
    key: K,
    val: V,
    prev: Option<Weak<RefCell<LinkedListNode<K, V>>>>,
    next: Link<K, V>,
}

impl<K, V> LinkedListNode<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }

    fn detach(&mut self) {
        if let Some(n) = self.next.take() {
            n.borrow_mut().prev = None;
        }
        if let Some(p) = self.prev.take().and_then(|x| x.upgrade()) {
            p.borrow_mut().next = None;
        }
    }

    fn link(prev_node: Node<K, V>, next_node: Node<K, V>) {
        prev_node.borrow_mut().next = Some(next_node.clone());
        next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
    }
}

type MyNode = Node<i32, HashSet<String>>;

struct AllOne {
head: MyNode,
    tail: MyNode,
    key2node: HashMap<String, MyNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        let mut head = Rc::new(RefCell::new(LinkedListNode::new(0, HashSet::new())));
        let mut tail = Rc::new(RefCell::new(LinkedListNode::new(0, HashSet::new())));
        LinkedListNode::link(head.clone(), tail.clone());
        Self {
            head,
            tail,
            key2node: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let mut cnt = 0;
        let (mut prev, mut next) = (self.head.clone(), self.head.borrow().next.clone().unwrap());  

        if let Some(nd) = self.key2node.get(&key).cloned() {
            self.key2node.remove(&key);
            let mut b = nd.borrow_mut();
            b.val.remove(&key);
            cnt = b.key;

            prev = b.prev.clone().unwrap().upgrade().unwrap();
            next = b.next.clone().unwrap();

            if b.key != 0 && b.val.is_empty() {
                b.detach();
                LinkedListNode::link(prev.clone(), next.clone()); 
            } else {
                prev = nd.clone();
            }
        }

        if next.borrow().key == cnt + 1 {
            next.borrow_mut().val.insert(key.clone());
            self.key2node.insert(key, next.clone());
        } else {
            let mut cur = Rc::new(RefCell::new(LinkedListNode::new(cnt + 1, HashSet::new())));
            cur.borrow_mut().val.insert(key.clone());
            self.key2node.insert(key, cur.clone());
            LinkedListNode::link(prev, cur.clone());
            LinkedListNode::link(cur, next);
        };
    }

    fn dec(&mut self, key: String) {
        let mut nd = self.key2node.get(&key).unwrap().clone();
        self.key2node.remove(&key);

        let mut b = nd.borrow_mut();
        let cnt = b.key;
        b.val.remove(&key);

        let (mut prev, mut next) = (
            b.prev.clone().unwrap().upgrade().unwrap(),
            b.next.clone().unwrap(),
        );

        if b.key != 0 && b.val.is_empty() {
            b.detach();
            LinkedListNode::link(prev.clone(), next.clone());
        } else {
            next = nd.clone();
        }

        if cnt - 1 == 0 {
            return;
        }

        drop(b);

        if prev.borrow().key == cnt - 1 {
            prev.borrow_mut().val.insert(key.clone());
            self.key2node.insert(key, prev.clone());
        } else {
            let mut cur = Rc::new(RefCell::new(LinkedListNode::new(cnt - 1, HashSet::new())));
            cur.borrow_mut().val.insert(key.clone());
            self.key2node.insert(key, cur.clone());
            LinkedListNode::link(prev, cur.clone());
            LinkedListNode::link(cur, next);
        };
    }

    fn get_max_key(&self) -> String {
       self.tail            .borrow()
            .prev
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .val
            .iter().next().unwrap_or(&String::new()).clone()
        
    }

    fn get_min_key(&self) -> String {
         self
            .head
            .borrow()
            .next
            .as_ref()
            .unwrap()
            .borrow()
            .val
            .iter().next().unwrap_or(&String::new()).clone()
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
// @lc code=end

