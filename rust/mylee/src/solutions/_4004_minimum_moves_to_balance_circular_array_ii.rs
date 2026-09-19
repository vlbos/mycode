// [4004. Minimum Moves to Balance Circular Array II](https://leetcode.com/problems/minimum-moves-to-balance-circular-array-ii/)

// You are given a circular array `balance` of length `n`, where `balance[i]` is the net balance of person `i`.
// In one move, a person can transfer **exactly** 1 unit of balance to either their left or right neighbor.
// Return the **minimum** number of moves required so that every person has a **non-negative** balance. If it is impossible, return -1.
// **Example 1:**
// **Input:** balance = [-1,2,-1]
// **Output:** 2
// **Explanation:**
// One optimal sequence of moves is:
// * Move 1 unit from `i = 1` to `i = 0`, resulting in `balance = [0, 1, -1]`
// * Move 1 unit from `i = 1` to `i = 2`, resulting in `balance = [0, 0, 0]`
// Thus, the minimum number of moves required is 2.
// **Example 2:**
// **Input:** balance = [4,-1,-2]
// **Output:** 3
// **Explanation:**
// One optimal sequence of moves is:
// * Move 1 unit from `i = 0` to `i = 1`, resulting in `balance = [3, 0, -2]`
// * Move 1 unit from `i = 0` to `i = 2`, resulting in `balance = [2, 0, -1]`
// * Move 1 unit from `i = 0` to `i = 2`, resulting in `balance = [1, 0, 0]`
// Thus, the minimum number of moves required is 3.
// **Example 3:**
// **Input:** balance = [-3,-3,5]
// **Output:** -1
// **Explanation:**
// It is impossible to make all balances non-negative for `balance = [-3, -3, 5]`, so the answer is -1.
// **Constraints:**
// * `1 \<= n == balance.length \<= 1000`
// * `-105 \<= balance[i] \<= 105`

impl Solution {
    pub fn min_moves(balance: Vec<i32>) -> i64 {
        let n = balance.len();
        let total = balance.iter().fold(0, |s, &v| s + v as i64);
        if total < 0 {
            return -1;
        }
        let (s, t, n2) = (n, n + 1, n + 2);
        let mut g = vec![vec![]; n2];
        let add_edge =
            |u: usize, v: usize, cap: i64, cost: i64, g: &mut Vec<Vec<(usize, i64, i64, i32)>>| {
                let (un, vn) = (g[u].len() as i32, g[v].len() as i32);
                g[u].push((v, cap, cost, vn));
                g[v].push((u, 0, -cost, un));
            };
        const INF_CAP: i64 = 1 << 50;
        for i in 0..n {
            let j = (i + 1) % n;
            add_edge(i, j, INF_CAP, 1, &mut g);
            add_edge(j, i, INF_CAP, 1, &mut g);
        }
        let mut demand_sum = 0;
        for i in 0..n {
            if balance[i] > 0 {
                add_edge(s, i, balance[i] as i64, 0, &mut g);
            } else if balance[i] < 0 {
                add_edge(i, t, -balance[i] as i64, 0, &mut g);
                demand_sum -= balance[i] as i64;
            }
        }

        let (mut flow, mut cost, mut potential, mut dist, mut pv_v, mut pv_e) = (
            0i64,
            0i64,
            vec![0i64; n2],
            vec![0i64; n2],
            vec![0; n2],
            vec![0; n2],
        );
        const INF: i64 = 1 << 60;
        let maxf = demand_sum;
        use std::cmp::Reverse;
        while flow < maxf {
            let mut q = std::collections::BinaryHeap::from([Reverse((0, s))]);
            dist.fill(INF);
            dist[s] = 0;
            while let Some(Reverse((d, u))) = q.pop() {
                if d != dist[u] {
                    continue;
                }
                for (i, &(to, cap, cost, rev)) in g[u].iter().enumerate() {
                    if cap <= 0 {
                        continue;
                    }
                    let nd = dist[u] + cost + potential[u] - potential[to];
                    if dist[to] > nd {
                        dist[to] = nd;
                        pv_v[to] = u;
                        pv_e[to] = i;
                        q.push(Reverse((nd, to)));
                    }
                }
            }
            if dist[t] == INF {
                break;
            }
            for (p, &d) in potential.iter_mut().zip(&dist) {
                if d < INF {
                    *p += d;
                }
            }
            let mut addf = maxf - flow;
            let mut v = t;
            while v != s {
                addf = addf.min(g[pv_v[v]][pv_e[v]].1);
                v = pv_v[v];
            }
            v = t;
            while v != s {
                let rev = g[pv_v[v]][pv_e[v]].3;
                g[pv_v[v]][pv_e[v]].1 -= addf;
                g[v][rev as usize].1 += addf;
                v = pv_v[v];
            }
            let add_cost = potential[t] - potential[s];
            flow += addf;
            cost += addf * add_cost;
        }
        if flow < demand_sum {
            return -1;
        }
        cost
    }
}

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_moves_1() {
        assert_eq!(2, Solution::min_moves(vec![-1, 2, -1]));
    }
    #[test]
    pub fn test_min_moves_2() {
        assert_eq!(3, Solution::min_moves(vec![4, -1, -2]));
    }
    #[test]
    pub fn test_min_moves_3() {
        assert_eq!(-1, Solution::min_moves(vec![-3, -3, 5]));
    }
}
