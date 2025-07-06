// [3383\. Minimum Runes to Add to Cast Spell 🔒](https://leetcode.com/problems/minimum-runes-to-add-to-cast-spell)
// ================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Alice has just graduated from wizard school, and wishes to cast a magic spell to celebrate. The magic spell contains certain **focus points** where magic needs to be concentrated, and some of these focus points contain **magic crystals** which serve as the spell's energy source. Focus points can be linked through **directed runes**, which channel magic flow from one focus point to another.

// You are given a integer `n` denoting the _number_ of focus points and an array of integers `crystals` where `crystals[i]` indicates a focus point which holds a magic crystal. You are also given two integer arrays `flowFrom` and `flowTo`, which represent the existing **directed runes**. The `ith` rune allows magic to freely flow from focus point `flowFrom[i]` to focus point `flowTo[i]`.

// You need to find the number of directed runes Alice must add to her spell, such that _each_ focus point either:

// *   **Contains** a magic crystal.
// *   **Receives** magic flow _from_ another focus point.

// Return the **minimum** number of directed runes that she should add.

// **Example 1:**

// **Input:** n = 6, crystals = \[0\], flowFrom = \[0,1,2,3\], flowTo = \[1,2,3,0\]

// **Output:** 2

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample0.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample0.png)

// Add two directed runes:

// *   From focus point 0 to focus point 4.
// *   From focus point 0 to focus point 5.

// **Example 2:**

// **Input:** n = 7, crystals = \[3,5\], flowFrom = \[0,1,2,3,5\], flowTo = \[1,2,0,4,6\]

// **Output:** 1

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample1.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3383.Minimum%20Runes%20to%20Add%20to%20Cast%20Spell/images/runesexample1.png)

// Add a directed rune from focus point 4 to focus point 2.

// **Constraints:**

// *   `2 <= n <= 105`
// *   `1 <= crystals.length <= n`
// *   `0 <= crystals[i] <= n - 1`
// *   `1 <= flowFrom.length == flowTo.length <= min(2 * 105, (n * (n - 1)) / 2)`
// *   `0 <= flowFrom[i], flowTo[i] <= n - 1`
// *   `flowFrom[i] != flowTo[i]`
// *   All pre-existing directed runes are **distinct**.

// int min_runes_to_add(int n, vector<int>& crystals, vector<int>& flow_from, vector<int>& flow_to) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_runes_to_add(
        n: i32,
        crystals: Vec<i32>,
        flow_from: Vec<i32>,
        flow_to: Vec<i32>,
    ) -> i32 {
        use std::collections::VecDeque;
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for (a, b) in flow_from.into_iter().zip(flow_to) {
            g[a as usize].push(b as usize);
            g[b as usize].push(a as usize);
        }
        let bfs = |mut q: VecDeque<usize>, vis: &mut Vec<i32>| {
            while let Some(a) = q.pop_front() {
                for &b in &g[a] {
                    if vis[b] != 1 {
                        vis[b] = 1;
                        q.push_back(b);
                    }
                }
            }
        };
        fn dfs(a: usize, g: &Vec<Vec<usize>>, vis: &mut Vec<i32>, seq: &mut Vec<usize>) {
            vis[a] = 2;
            for &b in &g[a] {
                if vis[b] == 0 {
                    dfs(b, g, vis, seq);
                }
            }
            seq.push(a);
        }

        let mut vis = vec![0; n];
        for &x in &crystals {
            vis[x as usize] = 1;
        }
        let mut q: VecDeque<_> = crystals.into_iter().map(|i| i as usize).collect();
        bfs(q, &mut vis);
        let mut seq = vec![];
        for i in 0..n {
            if vis[i] == 0 {
                dfs(i, &g, &mut vis, &mut seq);
            }
        }
        seq.reverse();
        let mut ans = 0;
        for i in seq {
            if vis[i] == 2 {
                vis[i] = 1;
                bfs(VecDeque::from([i]), &mut vis);
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_runes_to_add_1() {
        assert_eq!(
            2,
            Solution::min_runes_to_add(6, vec![0], vec![0, 1, 2, 3], vec![1, 2, 3, 0])
        );
    }
    #[test]
    pub fn test_min_runes_to_add_2() {
        assert_eq!(
            1,
            Solution::min_runes_to_add(7, vec![3, 5], vec![0, 1, 2, 3, 5], vec![1, 2, 0, 4, 6])
        );
    }
}
