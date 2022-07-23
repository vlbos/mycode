// 356\. Line Reflection
// =====================

// Given n points on a 2D plane, find if there is such a line parallel to y-axis that reflect the given points.

// **Example 1:**

// **Input:** \[\[1,1\],\[-1,1\]\]
// **Output:** true

// **Example 2:**

// **Input:** \[\[1,1\],\[-1,-1\]\]
// **Output:** false

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
        // let points = points
        //     .into_iter()
        //     .map(|v| (v[0], v[1]))
        //     .collect::<HashSet<_>>();
        // let mut counter: HashMap<i32, (i32, i32)> = HashMap::new();
        // points.iter().for_each(|p| {
        //     counter
        //         .entry(p.1)
        //         .and_modify(|h| {
        //             h.0 += p.0;
        //             h.1 += 1;
        //         })
        //         .or_insert((p.0, 1));
        // });
        // let mut last_axis_2x: Option<i32> = None;
        // for (_, v) in counter {
        //     let count = v.1;
        //     let sum = v.0;
        //     let axis_2x = sum * 2 / count;
        //     if let Some(la2x) = last_axis_2x {
        //         if la2x != axis_2x {
        //             return false;
        //         }
        //     }
        //     last_axis_2x = Some(axis_2x);
        // }
        // let axis_2x = last_axis_2x.unwrap();
        // let mut pair = HashMap::<(i32, i32), usize>::new();
        // points.iter().for_each(|v| {
        //     let x = v.0;
        //     let y = v.1;
        //     if 2 * x != axis_2x {
        //         let me = (x, y);
        //         let another = (axis_2x - x, y);
        //         if let Some(&count) = pair.get(&me) {
        //             if count == 1 {
        //                 pair.remove(&me);
        //             } else {
        //                 pair.insert(me, count - 1);
        //             }
        //         } else {
        //             pair.entry(another).and_modify(|v| *v += 1).or_insert(1);
        //         }
        //     }
        // });
        // pair.is_empty()
        let mut pair = std::collections::HashSet::new();
        for point in &points {
            if pair.contains(point) {
                return true;
            }
            pair.insert(vec![-point[0], point[1]]);
        }
        false
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_reflection_1() {
        assert!(Solution::is_reflected(vec![vec![1, 1], vec![-1, 1]]));
    }

    #[test]
    fn test_line_reflection_2() {
        assert!(!Solution::is_reflected(vec![vec![1, 1], vec![-1, -1]]));
    }
}
