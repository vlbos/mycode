// # 3656. Determine if a Simple Graph Exists 🔒

// Description
// -----------

// You are given an integer array `degrees`, where `degrees[i]` represents the desired degree of the `ith` vertex.

// Your task is to determine if there exists an **undirected simple** graph with **exactly** these vertex degrees.

// A **simple** graph has no self-loops or parallel edges between the same pair of vertices.

// Return `true` if such a graph exists, otherwise return `false`.

// **Example 1:**

// **Input:** degrees = \[3,1,2,2\]

// **Output:** true

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3600-3699/3656.Determine%20if%20a%20Simple%20Graph%20Exists/images/screenshot-2025-08-13-at-24347-am.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3600-3699/3656.Determine%20if%20a%20Simple%20Graph%20Exists/images/screenshot-2025-08-13-at-24347-am.png)​​​​​​​

// One possible undirected simple graph is:

// *   Edges: `(0, 1), (0, 2), (0, 3), (2, 3)`
// *   Degrees: `deg(0) = 3`, `deg(1) = 1`, `deg(2) = 2`, `deg(3) = 2`.

// **Example 2:**

// **Input:** degrees = \[1,3,3,1\]

// **Output:** false

// **Explanation:**​​​​​​​

// *   `degrees[1] = 3` and `degrees[2] = 3` means they must be connected to all other vertices.
// *   This requires `degrees[0]` and `degrees[3]` to be at least 2, but both are equal to 1, which contradicts the requirement.
// *   Thus, the answer is `false`.

// **Constraints:**

// *   `1 <= n == degrees.length <= 10​​​​​​​5`
// *   `0 <= degrees[i] <= n - 1`

// //  bool simple_graph_exists(vector<int>& degrees) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn simple_graph_exists(mut degrees: Vec<i32>) -> bool {
        let total = degrees.iter().fold(0, |s, &x| s + x as i64);
        if total % 2 != 0 {
            return false;
        }
        degrees.sort_unstable_by_key(|&x| -x);
        let (mut l, mut suf1, mut suf2) = (0, total, 0);
        let mut i = degrees.len();
        for (k, &d) in degrees.iter().enumerate() {
            let k = k as i64 + 1;
            l += d as i64;
            suf1 -= d as i64;
            while i > 0 && (degrees[i - 1] as i64) < k {
                suf2 += degrees[i - 1] as i64;
                i -= 1;
            }
            let i1 = i as i64;
            let r = k * (k - 1) + if i1 > k { (i1 - k) * k + suf2 } else { suf1 };
            if l > r {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_simple_graph_exists_1() {
        assert!(Solution::simple_graph_exists(vec![3, 1, 2, 2]));
    }
    #[test]
    pub fn test_simple_graph_exists_2() {
        assert!(!Solution::simple_graph_exists(vec![1, 3, 3, 1]));
    }
}
