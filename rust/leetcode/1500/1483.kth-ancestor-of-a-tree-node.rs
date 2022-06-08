/*
 * @lc app=leetcode id=1483 lang=rust
 *
 * [1483] Kth Ancestor of a Tree Node
 */

// @lc code=start
struct TreeAncestor {
    dp: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let mut dp = vec![Vec::new(); n];
        for i in 0..n {
            dp[i].push(parent[i]);
        }
        let mut j = 1;

        loop {
            let mut all_neg = true;
            for i in 0..n {
                let t = if dp[i][j - 1] != -1 {
                    dp[dp[i][j - 1] as usize][j - 1]
                } else {
                    -1
                };
                dp[i].push(t);
                if t != -1 {
                    all_neg = false;
                }
            }
            if all_neg {
                break;
            }
            j += 1;
        }
        Self { dp }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut ans = node;
        let mut pos = 0;
        let mut k = k;
        while k > 0 && ans != -1 {
            if pos >= self.dp[ans as usize].len() {
                return -1;
            }
            if k & 1 > 0 {
                ans = self.dp[ans as usize][pos];
            }
            k >>= 1;
            pos += 1;
        }
        ans
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */
// @lc code=end
