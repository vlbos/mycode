/*
 * @lc app=leetcode id=1172 lang=rust
 *
 * [1172] Dinner Plate Stacks
 */

// @lc code=start
const n: i32 = 100000;

struct DinnerPlates {
    stack: Vec<Vec<i32>>,
    capacity: i32,
    len: i32,
    tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            len: 0,
            stack: vec![Vec::new(); n as usize + 10],
            tree: vec![0; n as usize + 10],
        }
    }
    fn lowbit(&self, x: i32) -> i32 {
        x & -x
    }
    fn add(&mut self, x: i32, v: i32) {
        let mut i = x;
        while i <= n {
            self.tree[i as usize] += v;
            i += self.lowbit(i);
        }
    }
    fn get(&self, x: i32) -> i32 {
        let mut ans = 0;
        let mut i = x;
        while i > 0 {
            ans += self.tree[i as usize];
            i -= self.lowbit(i);
        }
        ans
    }
    fn get_pop(&self) -> i32 {
        let (mut l, mut r) = (1, n);
        while l <= r {
            let mid = (l + r) / 2;
            if if mid==1{0}else{self.get(mid-1)} >=  self.get(n) {
                r = mid-1;
            } else {
                l = mid + 1;
            }
        }
        r
    }
    fn get_push(&self) -> i32 {
        let (mut l, mut r) = (1, n);
        while l <= r {
            let mid = (l + r) / 2;
            if self.capacity as i64 * mid as i64== self.get(mid) as i64 {
                l = mid + 1;
               
            } else {
                 r = mid-1;
            }
        }
        l
    }
    fn push(&mut self, val: i32) {
        let p = self.get_push();
        self.stack[p as usize].push(val);
        self.len += 1;
        self.add(p, 1);
    }

    fn pop(&mut self) -> i32 {
        let p = self.get_pop();
        if p==0{
            return -1;
        }
        let r = self.stack[p as usize].pop().unwrap();
        self.len -= 1;
        self.add(p, -1);
        r
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let p = index as usize + 1;
        if self.stack[p].is_empty() {
            return -1;
        }
        let r = self.stack[p].pop().unwrap();
        self.len -= 1;
        self.add(index + 1, -1);
        r
    }
}


/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */
// @lc code=end
