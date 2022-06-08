/*
 * @lc app=leetcode id=587 lang=rust
 *
 * [587] Erect the Fence
 */

// @lc code=start
impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut hull = HashSet::new();
        if trees.len() < 4 {
            return trees;
        }
        let mut left_most = 0;
        for (i, tree) in trees.iter().enumerate() {
            if tree[0] < trees[left_most][0] {
                left_most = i;
            }
        }
        let mut p = left_most;
        let n = trees.len();
        let orientation = |p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>| {
            (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
        };
        let in_between = |p: &Vec<i32>, i: &Vec<i32>, q: &Vec<i32>| {
            (i[0] >= p[0] && i[0] <= q[0] || i[0] <= p[0] && i[0] >= q[0])
                && (i[1] >= p[1] && i[1] <= q[1] || i[1] <= p[1] && i[1] >= q[1])
        };
        loop {
            let mut q = (p + 1) % n;
            for (i, tree) in trees.iter().enumerate() {
                if orientation(&trees[p], tree, &trees[q]) < 0 {
                    q = i;
                }
            }
            for (i, tree) in trees.iter().enumerate() {
                if i != p
                    && i != q
                    && orientation(&trees[p], tree, &trees[q]) == 0
                    && in_between(&trees[p], tree, &trees[q])
                {
                    hull.insert(tree.clone());
                }
            }
            hull.insert(trees[q].clone());
            p = q;
            if p == left_most {
                break;
            }
        }
        hull.into_iter().collect::<Vec<Vec<i32>>>()
    }
}
// @lc code=end
