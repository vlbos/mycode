// 1182\. Shortest Distance to Target Color
// ========================================

// You are given an array `colors`, in which there are three colors: `1`, `2` and `3`.

// You are also given some queries. Each query consists of two integers `i` and `c`,
// return the shortest distance between the given index `i` and the target color `c`. If there is no solution return `-1`.

// **Example 1:**

// **Input:** colors = \[1,1,2,1,3,2,2,3,3\], queries = \[\[1,3\],\[2,2\],\[6,1\]\]
// **Output:** \[3,0,3\]
// **Explanation:**
// The nearest 3 from index 1 is at index 4 (3 steps away).
// The nearest 2 from index 2 is at index 2 itself (0 steps away).
// The nearest 1 from index 6 is at index 3 (3 steps away).

// **Example 2:**

// **Input:** colors = \[1,2\], queries = \[\[0,3\]\]
// **Output:** \[-1\]
// **Explanation:** There is no 3 in the array.

// **Constraints:**

// *   `1 <= colors.length <= 5*10^4`
// *   `1 <= colors[i] <= 3`
// *   `1 <= queries.length <= 5*10^4`
// *   `queries[i].length == 2`
// *   `0 <= queries[i][0] < colors.length`
// *   `1 <= queries[i][1] <= 3`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};
        let mut m = HashMap::new();
        for (i, &c) in colors.iter().enumerate() {
            m.entry(c).or_insert(BTreeSet::new()).insert(i as i32);
        }
        let mut ans = Vec::new();
        for q in &queries {
            if let Some(s) = m.get(&q[1]) {
                ans.push(
                    (*s.range(..q[0]).next_back().unwrap_or(&(i32::MAX / 2)) - q[0])
                        .abs()
                        .min((*s.range(q[0]..).next().unwrap_or(&(i32::MAX / 2)) - q[0]).abs()),
                );
            } else {
                ans.push(-1);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shortest_distance_color_1() {
        assert_eq!(
            vec![3, 0, 3],
            Solution::shortest_distance_color(
                vec![1, 1, 2, 1, 3, 2, 2, 3, 3],
                vec![vec![1, 3], vec![2, 2], vec![6, 1]]
            )
        );
    }
    #[test]
    pub fn test_shortest_distance_color_2() {
        assert_eq!(
            vec![-1],
            Solution::shortest_distance_color(vec![1, 2], vec![vec![0, 3]])
        );
    }
}
