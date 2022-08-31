// 1956\. Minimum Time For K Virus Variants to Spread[](https://leetcode.ca/2021-08-13-1956-Minimum-Time-For-K-Virus-Variants-to-Spread/#1956-minimum-time-for-k-virus-variants-to-spread)
// =======================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-13-1956-Minimum-Time-For-K-Virus-Variants-to-Spread/#level)
// -----------------------------------------------------------------------------------------------

// Hard

// Description[](https://leetcode.ca/2021-08-13-1956-Minimum-Time-For-K-Virus-Variants-to-Spread/#description)
// -----------------------------------------------------------------------------------------------------------

// There are `n` **unique** virus variants in an infinite 2D grid. You are given a 2D array `points`,
// where `points[i] = [x_i, y_i]` represents a virus originating at `(x_i, y_i)` on day `0`.
// Note that it is possible for **multiple** virus variants to originate at the **same** point.

// Every day, each cell infected with a virus variant will spread the virus to **all** neighboring points in the **four** cardinal directions
// (i.e. up, down, left, and right). If a cell has multiple variants, all the variants will spread without interfering with each other.

// Given an integer `k`, return _the **minimum integer** number of days for **any** point to contain at least `k` of the unique virus variants_.

// **Example 1:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/30/case-1.png)

// **Input:** points = \[\[1,1\],\[6,1\]\], k = 2

// **Output:** 3

// **Explanation:** On day 3, points (3,1) and (4,1) will contain both virus variants.

// **Example 2:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/30/case-2.png)

// **Input:** points = \[\[3,3\],\[1,2\],\[9,2\]\], k = 2

// **Output:** 2

// **Explanation:** On day 2, points (1,3), (2,3), (2,2), and (3,2) will contain the first two viruses.

// **Example 3:**

// ![Image text](https://assets.leetcode.com/uploads/2021/06/30/case-2.png)

// **Input:** points = \[\[3,3\],\[1,2\],\[9,2\]\], k = 3

// **Output:** 4

// **Explanation:** On day 4, the point (5,2) will contain all 3 viruses.

// **Constraints:**

// *   `n == points.length`
// *   `2 <= n <= 50`
// *   `points[i].length == 2`
// *   `1 <= x_i, y_i <= 10^9`
// *   `2 <= k <= n`

// Solution[](https://leetcode.ca/2021-08-13-1956-Minimum-Time-For-K-Virus-Variants-to-Spread/#solution)
// -----------------------------------------------------------------------------------------------------

// Use binary search. For a specific number of days, calculate whether there exists a point that contains at least `k` virus variants.

//     class Solution {
//         public int min_daysk_variants(int[][] points, int k) {
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_daysk_variants(points: Vec<Vec<i32>>, k: i32) -> i64 {
        let points: Vec<Vec<i32>> = points
            .into_iter()
            .map(|x| vec![x[0] + x[1], x[0] - x[1]])
            .collect();
        let mm = points
            .iter()
            .fold(vec![i32::MAX, i32::MIN, i32::MAX, i32::MIN], |a, p| {
                vec![
                    a[0].min(p[0]),
                    a[1].max(p[0]),
                    a[2].min(p[1]),
                    a[3].max(p[1]),
                ]
            });
        let (mut left, mut right) = (0, ((mm[1] - mm[0]) as i64 + (mm[3] - mm[2]) as i64 + 1) / 2);
        let check = |l: i64| {
            use std::collections::{HashMap, HashSet};
            let mut intervals = HashMap::new();
            let mut y_set = HashSet::new();
            for p in &points {
                let (p0, p1) = (p[0] as i64, p[1] as i64);
                let (x0, y0, x1, y1) = (p0 - l, p1 - l, p0 + l, p1 + l);
                *intervals
                    .entry(x0)
                    .or_insert(HashMap::new())
                    .entry(y0)
                    .or_insert(0) += 1;
                *intervals
                    .entry(x0)
                    .or_insert(HashMap::new())
                    .entry(y1 + 1)
                    .or_insert(0) -= 1;
                *intervals
                    .entry(x1 + 1)
                    .or_insert(HashMap::new())
                    .entry(y0)
                    .or_insert(0) -= 1;
                *intervals
                    .entry(x1 + 1)
                    .or_insert(HashMap::new())
                    .entry(y1 + 1)
                    .or_insert(0) += 1;
                y_set.insert(y0);
                y_set.insert(y1 + 1);
            }
            let mut sorted_x: Vec<i64> = intervals.iter().map(|x| *x.0).collect();
            sorted_x.sort();
            let mut sorted_y: Vec<i64> = y_set.iter().cloned().collect();
            sorted_y.sort();
            let mut count = HashMap::new();
            for &x in &sorted_x {
                for (&y, &c) in intervals.get(&x).unwrap_or(&HashMap::new()) {
                    *count.entry(y).or_insert(0) += c;
                }
                let mut cnt = 0;
                for &y in &sorted_y {
                    cnt += *count.get(&y).unwrap_or(&0);
                    if cnt >= k {
                        return true;
                    }
                }
            }
            false
        };
        while left <= right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_daysk_variants_1() {
        assert_eq!(
            3,
            Solution::min_daysk_variants(vec![vec![1, 1], vec![6, 1]], 2)
        );
    }
    #[test]
    pub fn test_min_daysk_variants_2() {
        assert_eq!(
            2,
            Solution::min_daysk_variants(vec![vec![3, 3], vec![1, 2], vec![9, 2]], 2)
        );
    }
    #[test]
    pub fn test_min_daysk_variants_3() {
        assert_eq!(
            4,
            Solution::min_daysk_variants(vec![vec![3, 3], vec![1, 2], vec![9, 2]], 3)
        );
    }
}
