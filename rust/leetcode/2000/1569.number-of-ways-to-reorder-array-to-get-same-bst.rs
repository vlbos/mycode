/*
 * @lc app=leetcode id=1569 lang=rust
 *
 * [1569] Number of Ways to Reorder Array to Get Same BST
 */

// @lc code=start
struct UnionFind {
    parent: Vec<usize>,
    root: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            root: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find_set(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.find_set(self.parent[x]);
        self.parent[x]
    }
    fn get_root(&mut self, x: usize) -> usize {
        let i = self.find_set(x);
        self.root[i]
    }
    fn unite(&mut self, x: usize, y: usize) {
        self.root[y] = self.root[x];
        let (x, y) = if self.size[x] < self.size[y] {
            (y, x)
        } else {
            (x, y)
        };
        self.parent[y] = x;
        self.size[x] += self.size[y];
    }
    fn find_and_unite(&mut self, x: usize, y: usize) -> bool {
        let (px, py) = (self.find_set(x), self.find_set(y));
        if px != py {
            self.unite(px, py);
            return true;
        }
        false
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    size: usize,
    ans: i32,
}
impl Node {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
            size: 1,
            ans: 0,
        }
    }
}
impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut fac = vec![0; n];
        let mut inv = vec![0; n];
        let mut facinv = vec![0; n];
        fac[0] = 1;
        fac[1] = 1;
        inv[0] = 1;
        inv[1] = 1;
        facinv[0] = 1;
        facinv[1] = 1;
        let p = 1_000_000_007;
        for i in 2..n {
            fac[i] = fac[i - 1] * i % p;
            inv[i] = (p - p / i) * inv[p % i] % p;
            facinv[i] = facinv[i - 1] * inv[i] % p;
        }
        let mut found = std::collections::HashMap::<usize,Option<Rc<RefCell<Node>>>>::new();
        let mut uf = UnionFind::new(n);
        for i in (0..n).rev() {
            let val = nums[i] as usize - 1;
            let mut node = Node::new();
            if val > 0 && found.contains_key(&(val - 1)) {
                let lchild = uf.get_root(val - 1);
                if let Some(l) = found.get(&lchild) {
                    node.left = l.clone();
                    node.size += l.as_ref().unwrap().borrow().size;
                }
                uf.find_and_unite(val, lchild);
            }
            if val < n - 1 && found.contains_key(&(val + 1)) {
                let rchild = uf.get_root(val + 1);
                if let Some(r) = found.get(&rchild) {
                    node.right = r.clone();
                    node.size += r.as_ref().unwrap().borrow().size;
                }
                uf.find_and_unite(val, rchild);
            }
            let (lsize, lans) = if let Some(l) = &node.left {
                (l.borrow().size, l.borrow().ans)
            } else {
                (0, 1)
            };
            let (rsize, rans) = if let Some(r) = &node.right {
                (r.borrow().size, r.borrow().ans)
            } else {
                (0, 1)
            };
            node.ans =( (fac[lsize + rsize] as i64 )* (facinv[lsize] as i64) %(p as i64) * (facinv[rsize] as i64)  %(p as i64) * (lans as i64)  %(p as i64) * (rans as i64) % (p as i64)) as i32;
            found.insert(val, Some(Rc::new(RefCell::new(node))));
        }
        let ans = found.get(&(nums[0] as usize - 1)).unwrap().as_ref().unwrap().borrow().ans;
        (ans- 1 + p as i32) % p as i32
    }
}
// @lc code=end
