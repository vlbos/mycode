// # [3973. Distinct Gate Paths to LCA 🔒](https://leetcode.com/problems/distinct-gate-paths-to-lca)

// ## Description

// You are given an undirected tree rooted at node 0 with `n` nodes numbered from 0 to `n - 1`,
// represented by an array `parent` where `parent[i]` is the parent of node `i`.

// Each node `i` has three types of gates,
// given in a 2D array `gates` where `gates[i] = [redi, bluei, whitei]` which represents the number of **red**, **blue**,
// and **white** gates at node `i`.

// - **Red** gate: usable only with a **red** card.
// - **Blue** gate: usable only with a **blue** card.
// - **White** gate: usable with **either** card, but **flips** the card color when used.

// Alice and Bob start at given nodes with either a red or blue card (`1` = red, `0` = blue).
// They must **independently** move **upward** to their **lowest common ancestor (LCA)**.

// At each node,
//  a person may move to their parent **only if** they can use **at least** one gate at that node with their current card.
//  **White** gates may be used any number of times to flip the card color.

// **Movement rules (one move = from `u` to `parent[u]`):**

// - Movement is only upward toward the root.
// - At node `u`, pick **exactly** one specific gate instance.
// Identical gates are treated as **separate** and counted individually.
// - If holding a **red** card: use a red gate to remain red, or a white gate to **change** to blue.
// - If holding a **blue** card: use a blue gate to remain blue, or a white gate to **change** to red.
// - If no usable gate exists at `u`, the sequence ends.

// You are also given a 2D array `queries` where `queries[i] = [aNodei, aCardi, bNodei, bCardi]`:

// - `aNodei`, `aCardi`: Alice's starting node and card.
// - `bNodei`, `bCardi`: Bob's starting node and card.

// For each query, count the number of **distinct** valid ways **modulo** `109 + 7` for both to reach their **LCA**.

// After computing the result for all queries, return the **bitwise XOR** of those values.

// **Note:**

// - Two ways are distinct if the set of gates used **differs** for either Alice or Bob.
// - If any person is already at the **LCA**, then the number of ways for them is 1.
// - The **lowest common ancestor (LCA)** is defined between two nodes `a` and `b` as the lowest node in a tree that has both `a` and `b` as descendants (where a node is allowed to be a descendant of itself).

// **Example 1:**

// **Input:** n = 3, parent = [-1,0,0], gates = [[1,0,1],[0,1,1],[1,1,0]], queries = [[1,0,2,0],[1,1,2,0],[1,0,2,1]]

// **Output:** 1

// **Explanation:**

// | i | Alice [Node, Card] | Bob [Node, Card] | LCA | Alice Path | Bob Path | Alice Ways | Bob Ways | Total Ways |
// | --- | --- | --- | --- | --- | --- | --- | --- | --- |
// | 0 | [1, 0]: Blue | [2, 0]: Blue | 0 | 1 → 0 | 2 → 0 | 2 (1 Blue + 1 White at node 1) | 1 (1 Blue at node 2) | 2 × 1 = 2 |
// | 1 | [1, 1]: Red | [2, 0]: Blue | 0 | 1 → 0 | 2 → 0 | 1 (1 White at node 1) | 1 (1 Blue at node 2) | 1 × 1 = 1 |
// | 2 | [1, 0]: Blue | [2, 1]: Red | 0 | 1 → 0 | 2 → 0 | 2 (1 Blue + 1 White at node 1) | 1 (1 Red at node 2) | 2 × 1 = 2 |

// Thus, the XOR of all values: `2 XOR 1 XOR 2 = 1`.

// **Example 2:**

// **Input:** n = 3, parent = [-1,0,1], gates = [[0,1,2],[1,0,1],[0,0,3]], queries = [[2,0,1,0],[2,1,0,0],[1,1,2,1]]

// **Output:** 3

// **Explanation:**

// | i | Alice [Node, Card] | Bob [Node, Card] | LCA | Alice Path | Bob Path | Alice Ways | Bob Ways | Total Ways |
// | --- | --- | --- | --- | --- | --- | --- | --- | --- |
// | 0 | [2, 0]: Blue | [1, 0]: Blue | 1 | 2 → 1 | 1 | 3 (3 White at node 2) | 1 (no move) | 3 × 1 = 3 |
// | 1 | [2, 1]: Red | [0, 0]: Blue | 0 | 2 → 1 → 0 | 0 | 3 (3 White at node 2) × 1 (1 White at node 1) = 3 | 1 (no move) | 3 × 1 = 3 |
// | 2 | [1, 1]: Red | [2, 1]: Red | 1 | 1 | 2 → 1 | 1 (no move) | 3 (3 White at node 2) | 1 × 3 = 3 |

// Thus, the XOR of all values: `3 XOR 3 XOR 3 = 3`.

// **Constraints:**​​​​​​​

// - `2 <= n <= 2 * 104`
// - `n == parent.length == gates.length`
// - `parent[0] == -1`
// - `0 <= parent[i] < n` for `i` in `[1, n - 1]`
// - `gates[i] == [redi, bluei, whitei]`
// - `0 <= redi, bluei, whitei <= 10`
// - `1 <= queries.length <= 2 * 104`
// - `queries[i] = [aNodei, aCardi, bNodei, bCardi]`
// - `0 <= aNodei, bNodei <= n - 1`
// - `0 <= aCardi, bCardi <= 1`
// - The input is generated such that the array `parent` represents a valid tree.

//  int distinct_paths(int n, vector<int>& parent, vector<vector<int>>& gates, vector<vector<int>>& queries) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn distinct_paths(
        n: i32,
        parent: Vec<i32>,
        mut gates: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const BB: usize = 0;
        const BR: usize = 1;
        const RB: usize = 2;
        const RR: usize = 3;
        let ceil_log2 = |x: i32| 32 - (x - 1).leading_zeros();
        let mult = |x: &[i64], y: &[i64]| -> Vec<i64> {
            vec![
                (x[BB] * y[BB] + x[BR] * y[RB]) % MOD,
                (x[BB] * y[BR] + x[BR] * y[RR]) % MOD,
                (x[RB] * y[BB] + x[RR] * y[RB]) % MOD,
                (x[RB] * y[BR] + x[RR] * y[RR]) % MOD,
            ]
        };
        let nn = n as usize;
        let n11 = ceil_log2(n - 1) as usize + 1;
        let mut depth = vec![0; nn];
        let mut par = vec![vec![0i32; nn]; n11];
        let lca = |mut a: i32, mut b: i32, par: &Vec<Vec<i32>>, depth: &mut Vec<i32>| {
            if depth[a as usize] < depth[b as usize] {
                std::mem::swap(&mut a, &mut b);
            }
            let d = depth[a as usize] - depth[b as usize];
            for k in 0..par.len() {
                if d & (1 << k) != 0 {
                    a = par[k][a as usize];
                }
            }
            if a == b {
                return a;
            }

            for (k, p) in par.iter().enumerate().rev() {
                if p[a as usize] != p[b as usize] {
                    (a, b) = (p[a as usize], p[b as usize]);
                }
            }
            par[0][a as usize]
        };
        let mut cnt = vec![vec![vec![0; 4]; nn]; n11];
        let count = |mut u: i32,
                     card: i32,
                     t: i32,
                     depth: &[i32],
                     cnt: &Vec<Vec<Vec<i64>>>,
                     par: &Vec<Vec<i32>>| {
            if u == t {
                return 1;
            }

            let d = depth[u as usize] - depth[t as usize];
            let mut b = if card == 0 { 1 } else { 0 };
            let mut r = if card == 1 { 1 } else { 0 };
            for k in 0..par.len() {
                if d & (1 << k) == 0 {
                    continue;
                }
                (b, r) = (
                    (b * cnt[k][u as usize][BB] + r * cnt[k][u as usize][RB]) % MOD,
                    (b * cnt[k][u as usize][BR] + r * cnt[k][u as usize][RR]) % MOD,
                );
                u = par[k][u as usize];
            }
            (b + r) % MOD
        };
        let mut g = vec![vec![]; nn];
        for (u, &p) in parent.iter().enumerate() {
            if p != -1 {
                g[p as usize].push(u);
            }
        }
        let mut s: Vec<usize> = vec![0];
        while let Some(u) = s.pop() {
            for &v in &g[u] {
                depth[v] = depth[u] + 1;
                s.push(v);
            }
        }
        par[0] = parent.clone();
        cnt[0] = gates
            .iter()
            .map(|gate| {
                vec![
                    gate[1] as i64,
                    gate[2] as i64,
                    gate[2] as i64,
                    gate[0] as i64,
                ]
            })
            .collect();
        for k in 1..par.len() {
            for u in 0..nn {
                // if par[k - 1][u] == -1 {
                //     continue;
                // }
                // par[k][u] = par[k - 1][par[k - 1][u] as usize];
                // let v = mult(&cnt[k - 1][u], &cnt[k - 1][par[k - 1][u] as usize]);
                // cnt[k][u] = v;
                let p_prev = par[k - 1][u];
                if p_prev == -1 {
                    par[k][u] = -1;
                } else {
                    par[k][u] = par[k - 1][p_prev as usize];
                    if par[k][u] != -1 {
                        cnt[k][u] = mult(&cnt[k - 1][u], &cnt[k - 1][p_prev as usize]);
                    }
                }
            }
        }
        let mut ans = 0;
        for q in queries {
            let l = lca(q[0], q[2], &par, &mut depth);
            // println!("{},{},{l}",q[0] , q[2]);
            ans ^= count(q[0], q[1], l, &depth, &cnt, &par)
                * count(q[2], q[3], l, &depth, &cnt, &par)
                % MOD;
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_distinct_paths_1() {
        assert_eq!(
            1,
            Solution::distinct_paths(
                3,
                vec![-1, 0, 0],
                lc_matrix![[1, 0, 1], [0, 1, 1], [1, 1, 0]],
                lc_matrix![[1, 0, 2, 0], [1, 1, 2, 0], [1, 0, 2, 1]]
            )
        );
    }
    #[test]
    pub fn test_distinct_paths_2() {
        assert_eq!(
            3,
            Solution::distinct_paths(
                3,
                vec![-1, 0, 1],
                lc_matrix![[0, 1, 2], [1, 0, 1], [0, 0, 3]],
                lc_matrix![[2, 0, 1, 0], [2, 1, 0, 0], [1, 1, 2, 1]]
            )
        );
    }
}
