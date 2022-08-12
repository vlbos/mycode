// 624\. Maximum Distance in Arrays
// ================================

// Given `m` arrays, and each array is sorted in ascending order. Now you can pick up two integers from two different arrays (each array picks one) and calculate the distance.
// We define the distance between two integers `a` and `b` to be their absolute difference `|a-b|`. Your task is to find the maximum distance.

// **Example 1:**

// **Input:**
// \[\[1,2,3\],
//  \[4,5\],
//  \[1,2,3\]\]
// **Output:** 4
// **Explanation:**
// One way to reach the maximum distance 4 is to pick 1 in the first or third array and pick 5 in the second array.

// **Note:**

// 1.  Each given array will have at least 1 number. There will be at least two non-empty arrays.
// 2.  The total number of the integers in **all** the `m` arrays will be in the range of \[2, 10000\].
// 3.  The integers in the `m` arrays will be in the range of \[-10000, 10000\].

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Yahoo](https://leetcode.ca/tags/#Yahoo)

// @lc code=start
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;

impl Solution {
    pub fn   max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        // let len = arrays.len();
        // if len <= 1 {
        //     return 0;
        // }
        // let mut mins = BinaryHeap::<(i32, usize)>::new();
        // let mut maxs = BinaryHeap::<Reverse<(i32, usize)>>::new();
        // for i in 0..len {
        //     let a = &arrays[i];
        //     let min_add = if mins.len() < 2 {
        //         (a[0], i)
        //     } else {
        //         let last_min_2 = mins.pop().unwrap();
        //         if a[0] < last_min_2.0 {
        //             (a[0], i)
        //         } else {
        //             last_min_2
        //         }
        //     };
        //     mins.push(min_add);
        //     let max_add = Reverse(if maxs.len() < 2 {
        //         (a[a.len() - 1], i)
        //     } else {
        //         let Reverse(last_max_2) = maxs.pop().unwrap();
        //         if a[a.len() - 1] > last_max_2.0 {
        //             (a[a.len() - 1], i)
        //         } else {
        //             last_max_2
        //         }
        //     });
        //     maxs.push(max_add);
        // }
        // let maxs = maxs.into_iter().map(|Reverse(v)| v).collect::<Vec<_>>();
        // let mins = mins.into_iter().collect::<Vec<_>>();
        // if maxs[1].1 != mins[1].1 {
        //     i32::abs(maxs[1].0 - mins[1].0)
        // } else {
        //     i32::max(
        //         i32::abs(maxs[0].0 - mins[1].0),
        //         i32::abs(maxs[1].0 - mins[0].0),
        //     )
        // }
        let mut min: Vec<(i32, usize)> = arrays
            .iter()
            .enumerate()
            .map(|(i, x)| (*x.iter().min().unwrap(), i))
            .collect();
        let mut max: Vec<(i32, usize)> = arrays
            .iter()
            .enumerate()
            .map(|(i, x)| (*x.iter().max().unwrap(), i))
            .collect();
        min.sort_by_key(|x| x.0);
        max.sort_by(|a, b| b.0.cmp(&a.0));
        if min[0].1 != max[0].1 {
            (min[0].0 - max[0].0).abs()
        } else {
            (min[0].0 - max[1].0).abs().max((min[1].0 - max[0].0).abs())
        }
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
   pub fn  test_max_distance_1() {
        assert_eq!(
            Solution::max_distance(lc_matrix![[1, 2, 3], [0, 4, 5], [1, 2, 3]]),
            4
        );
    }
}
