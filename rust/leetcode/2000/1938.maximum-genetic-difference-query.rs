/*
 * @lc app=leetcode id=1938 lang=rust
 *
 * [1938] Maximum Genetic Difference Query
 */

// @lc code=start
struct Trie {
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
    cnt: i32,
}
impl Trie {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
            cnt: 0,
        }
    }
}

const MAXD: u32 = 17;

impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = parents.len();
        let mut edges = vec![Vec::new(); n];
        let mut root = -1;
        for (i, &p) in parents.iter().enumerate() {
            if p == -1 {
                root = i as i32;
            } else {
                edges[p as usize].push(i);
            }
        }
        let m = queries.len();
        let mut stored = vec![Vec::new(); n];
        let mut ans = vec![0; m];
        for (i, q) in queries.iter().enumerate() {
            stored[q[0] as usize].push((i, q[1]));
        }
        let mut r = Trie::new();
        fn insert(x: i32, r: &mut Trie) {
            let mut cur = r;
            for i in (0..=MAXD).rev() {
                if x & (1 << i) > 0 {
                    cur = cur
                        .right
                        .get_or_insert(Box::new(Trie::new()))
                        .as_mut();
                } else {
                    cur = cur
                        .left
                        .get_or_insert(Box::new(Trie::new()))
                        .as_mut();
                }
                cur.cnt += 1;
            }
        }
        fn query(x: i32, r: &Trie) -> i32 {
            let mut ans = 0;
            let mut cur = r;
            for i in (0..=MAXD).rev() {
                if x & (1 << i) > 0 {
                    if cur.left.is_some() && cur.left.as_ref().unwrap().cnt > 0 {
                        ans |= (1 << i);
                        cur = cur.left.as_ref().unwrap();
                    } else {
                        cur = cur.right.as_ref().unwrap();
                    }
                } else {
                    if cur.right.is_some() && cur.right.as_ref().unwrap().cnt > 0 {
                        ans |= (1 << i);
                        cur = cur.right.as_ref().unwrap();
                    } else {
                        cur = cur.left.as_ref().unwrap();
                    }
                }
            }
            ans
        }
        fn erase(x: i32, r: &mut Trie) {
            let mut cur = r;
            for i in (0..=MAXD).rev() {
                if x & (1 << i) > 0 {
                    cur = cur
                        .right.as_mut()
                        .unwrap()
                        ;
                } else {
                    cur = cur
                        .left.as_mut()
                        .unwrap()
                        ;
                }
                cur.cnt -= 1;
            }
        }
        fn dfs(
            u: i32,
            r: &mut Trie,
            stored: &Vec<Vec<(usize, i32)>>,
            edges: &Vec<Vec<usize>>,
            ans: &mut Vec<i32>,
        ) {
            insert(u, r);
            for &(idx, num) in &stored[u as usize] {
                ans[idx] = query(num, &*r);
            }
            for &v in &edges[u as usize] {
                dfs(v as i32, r, stored, edges, ans);
            }
            erase(u, r);
        }
        dfs(root, &mut r, &stored, &edges, &mut ans);
        ans
    }
}
// @lc code=end
