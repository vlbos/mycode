/*
 * @lc app=leetcode id=699 lang=rust
 *
 * [699] Falling Squares
 */

// @lc code=start
struct SegmentTree {
    n: i32,
    h: i32,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}
impl SegmentTree {
    pub fn new(n: usize) -> Self {
        let mut h = 1;
        while (1 << h) < n {
            h += 1;
        }
        Self {
            n: n as i32,
            h,
            tree: vec![0; 2 * n],
            lazy: vec![0; n],
        }
    }
    fn apply(&mut self, x: i32, val: i32) {
        self.tree[x as usize] = self.tree[x as usize].max(val);
        if x < self.n {
            self.lazy[x as usize] = self.lazy[x as usize].max(val);
        }
    }
    fn pull(&mut self, mut x: i32) {
        while x > 1 {
            x >>= 1;
            self.tree[x as usize] =
                self.tree[(x as usize) * 2].max(self.tree[(x as usize) * 2 + 1]);
            self.tree[x as usize] = self.tree[x as usize].max(self.lazy[x as usize]);
        }
    }
    fn push(&mut self, x: i32) {
        for h in (1..=self.h).rev() {
            let y = x >> h;
            if self.lazy[y as usize] > 0 {
                self.apply(y * 2, self.lazy[y as usize]);
                self.apply(y * 2 + 1, self.lazy[y as usize]);
                self.lazy[y as usize] = 0;
            }
        }
    }
    pub fn update(&mut self, mut l: i32, mut r: i32, h: i32) {
        l += self.n;
        r += self.n;
        let (l0, r0) = (l, r);

        while l <= r {
            if l & 1 == 1 {
                self.apply(l, h);
                l += 1;
            }
            if r & 1 == 0 {
                self.apply(r, h);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.pull(l0);
        self.pull(r0);
    }
    pub fn query(&mut self, mut l: i32, mut r: i32) -> i32 {
        l += self.n;
        r += self.n;
        let mut ans = 0;
        self.push(l);
        self.push(r);
        while l <= r {
            if l & 1 == 1 {
                ans = ans.max(self.tree[l as usize]);
                l += 1;
            }
            if r & 1 == 0 {
                ans = ans.max(self.tree[r as usize]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        ans
    }
}
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut coords: Vec<i32> = positions
            .iter()
            .map(|x| vec![x[0], x[0] + x[1] - 1])
            .flatten()
            .collect();
        coords.sort();
        coords.dedup();
        let index: std::collections::HashMap<i32, i32> = coords
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as i32))
            .collect();
        let mut best = 0;
        let mut ans = Vec::new();
        let mut tree = SegmentTree::new(index.len());
        for pos in &positions {
            let l = *index.get(&pos[0]).unwrap();
            let r = *index.get(&(pos[0] + pos[1] - 1)).unwrap();
            let h = tree.query(l, r) + pos[1];
            tree.update(l, r, h);
            best = best.max(h);
            ans.push(best);
        }
        ans
    }
}
// @lc code=end
