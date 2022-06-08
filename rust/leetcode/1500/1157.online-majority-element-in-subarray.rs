/*
 * @lc app=leetcode id=1157 lang=rust
 *
 * [1157] Online Majority Element In Subarray
 */

// @lc code=start
struct MajorityChecker {
    node: Vec<Vec<i32>>,
    root: Vec<i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
         let nn = 20000 + 100;
        let mut node = vec![vec![0; 3]; nn << 5];
        let mut root = vec![0; nn];
        let n = arr.len() as i32;
        let mut tot = 0;
        fn ins(
            l: i32,
            r: i32,
            pre: i32,
            now: &mut i32,
            num: i32,
            tot: &mut i32,
            node: &mut Vec<Vec<i32>>,
        ) {
            *tot += 1;
            *now = *tot;
            let (now, pre) = (*now as usize, pre as usize);
            node[now] = node[pre].clone();
            node[now][2] += 1;
            if l == r {
                return;
            }
            let mid = (l + r) / 2;
            if num <= mid {
                let mut v =node[now][0];
                ins(l, mid, node[pre][0], &mut v, num, tot, node);
                node[now][0]=v;
            } else {
                 let mut v =node[now][1];
                ins(mid + 1, r, node[pre][1],&mut  v, num, tot, node);
                 node[now][1]=v;
            }
        }
        for (i, &v) in arr.iter().enumerate() {
            ins(1, n, root[i], &mut root[i + 1], v, &mut tot, &mut node);
        }
        Self { node, root, n }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        fn search(
            l: i32,
            r: i32,
            start: i32,
            end: i32,
            num: i32,
            node: &Vec<Vec<i32>>,
        ) -> i32 {
            if l == r {
                return l;
            }
            let mid = (l + r) / 2;
            let (start, end) = (start as usize, end as usize);
            if node[node[end][0] as usize][2] - node[node[start][0] as usize][2] >= num {
                return search(l, mid, node[start][0], node[end][0], num, node);
            }
            if node[node[end][1] as usize][2] - node[node[start][1] as usize][2] >= num {
                return search(mid + 1, r, node[start][1], node[end][1], num, node);
            }
            -1
        }
        search(
            1,
            self.n,
            self.root[left as usize],
            self.root[right as usize + 1],
            threshold,
            &self.node,
        )
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */
// @lc code=end
