// # [2613. Beautiful Pairs](https://leetcode.com/problems/beautiful-pairs)

// ## Description

//  You are given two  0-indexed  integer arrays  nums1  and  nums2  of the same length.
// A pair of indices  (i,j)  is called  beautiful  if |nums1[i] - nums1[j]| + |nums2[i] - nums2[j]|
// is the smallest amongst all possible indices pairs where  i  < j .

//  Return  the beautiful pair. In the case that there are multiple beautiful pairs,
// return the lexicographically smallest pair.

//  Note that

// 	  |x|  denotes the absolute value of  x .
// 	 A pair of indices  (i 1 , j 1 )  is lexicographically smaller than  (i 2 , j 2 )
// if  i 1   < i 2   or  i 1  == i 2   and  j 1   < j 2  .

//   ### Example 1:

//  Input:  nums1 = [1,2,3,2,4], nums2 = [2,3,1,2,3]
//  Output:  [0,3]
//  Explanation:  Consider index 0 and index 3.
// The value of |nums1[i]-nums1[j]| + |nums2[i]-nums2[j]| is 1, which is the smallest value we can achieve.

//   ### Example 2:

//  Input:  nums1 = [1,2,4,3,2,5], nums2 = [1,4,2,3,5,1]
//  Output:  [1,4]
//  Explanation:  Consider index 1 and index 4.
// The value of |nums1[i]-nums1[j]| + |nums2[i]-nums2[j]| is 1, which is the smallest value we can achieve.

//   Constraints:

// 	  2  <= nums1.length, nums2.length  <= 10^5
// 	  nums1.length == nums2.length
// 	  0  <= nums1 i   <= nums1.length
// 	  0  <= nums2 i   <= nums2.length

// ## Solutions

// **Approach 1: Sorting + Divide and Conquer**

// This problem is equivalent to finding two points in the plane,
// such that the Manhattan distance between them is the smallest.
// If there are multiple points satisfying the condition, return the one with the smallest index.

// First, we handle the case where there are duplicate points.
// For each point, we record the corresponding indices in a list.
// If the length of the index list is greater than $1$,
// then the first two indices in the index list can be used as candidates, and we find the smallest index pair.

// If there are no duplicate points, we sort all the points by $x$ coordinates,
// and then use the divide and conquer to solve the problem.

// For each interval $[l, r]$, we first calculate the median of the $x$ coordinates $m$,
// and then recursively solve the left and right intervals, and get $d_1, (pi_1, pj_1)$ and $d_2, (pi_2, pj_2)$ respectively,
// where $d_1$ and $d_2$ are the minimum Manhattan distances of the left and right intervals respectively,
// and $(pi_1, pj_1)$ and $(pi_2, pj_2)$ are the index pairs of the two points of the minimum Manhattan distance of the left and right intervals respectively.
// We take the smaller one of $d_1$ and $d_2$ as the minimum Manhattan distance of the current interval,
// and if $d_1 = d_2$, we take the one with the smaller index as the answer.
// The corresponding two points of the index are taken as the answer.

// The above considers the case where the two points are on the same side.
// If the two points are on different sides, we take the middle point,
// i.e. the point with the index of $m = \lfloor (l + r) / 2 \rfloor$ as the standard, and divide a new region.
// The range of this region is to expand the range of $d_1$ from the middle point to the left and right sides respectively.
// Then we sort these points in the range by $y$ coordinates, and then traverse each point pair in the sorted order.
// If the difference of the $y$ coordinates of the two points is greater than the current minimum Manhattan distance,
//  then the following point pairs do not need to be considered, because their $y$ coordinate differences are larger,
// so the Manhattan distance is larger, and it will not be smaller than the current minimum Manhattan distance.
// Otherwise, we update the minimum Manhattan distance, and update the answer.

// Finally, we return the answer.

// Time complexity: $O(n \times \log n)$, where $n$ is the length of the array.

// Space complexity: $O(n)$.

// ### **C++**

// ```cpp
// class Solution {
// public:
//     vector  beautiful_pair(vector & nums1, vector & nums2) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn beautiful_pair(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut pl = HashMap::new();
        for (i, (&x, &y)) in nums1.iter().zip(&nums2).enumerate() {
            pl.entry((x, y)).or_insert(Vec::new()).push(i);
        }
        let mut points = Vec::new();
        for (i, (&x, &y)) in nums1.iter().zip(&nums2).enumerate() {
            if let Some(v) = pl.get(&(x, y)) {
                if v.len() > 1 {
                    return vec![i as i32, v[1] as i32];
                }
            }
            points.push(vec![x, y, i as i32]);
        }
        points.sort_unstable();
        fn dfs(l: i32, r: i32, points: &Vec<Vec<i32>>) -> Vec<i32> {
            if l >= r {
                return vec![i32::MAX, -1, -1];
            }
            let m = (l + r) >> 1;
            let x = points[m as usize][0];
            let mut d1 = dfs(l, m, points);
            let d2 = dfs(m + 1, r, points);
            if d1[0] > d2[0]
                || (d1[0] == d2[0] && (d1[1] > d2[1] || (d1[1] == d2[1] && d1[2] > d2[2])))
            {
                d1 = d2;
            }
            let mut t: Vec<_> = points[l as usize..=r as usize]
                .iter()
                .filter_map(|p| {
                    if (p[0] - x).abs() <= d1[0] {
                        Some(p.clone())
                    } else {
                        None
                    }
                })
                .collect();
            t.sort_unstable_by_key(|x| x[1]);
            let n = t.len();
            for i in 0..n {
                for j in i + 1..n {
                    if t[j][1] - t[i][1] > d1[0] {
                        break;
                    }
                    let (pi, pj) = if t[j][2] < t[i][2] {
                        (t[j][2], t[i][2])
                    } else {
                        (t[i][2], t[j][2])
                    };
                    let dist =
                        |x1: i32, y1: i32, x2: i32, y2: i32| (x1 - x2).abs() + (y1 - y2).abs();
                    let d = dist(t[i][0], t[i][1], t[j][0], t[j][1]);
                    if d < d1[0] || (d == d1[0] && (pi < d1[1] || (pi == d1[1] && pj < d1[2]))) {
                        d1 = vec![d, pi, pj];
                    }
                }
            }
            d1
        }

        dfs(0, points.len() as i32 - 1, &points)[1..].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_beautiful_pair_1() {
        assert_eq!(
            vec![0, 3],
            Solution::beautiful_pair(vec![1, 2, 3, 2, 4], vec![2, 3, 1, 2, 3])
        );
    }
    #[test]
    pub fn test_beautiful_pair_2() {
        assert_eq!(
            vec![1, 4],
            Solution::beautiful_pair(vec![1, 2, 4, 3, 2, 5], vec![1, 4, 2, 3, 5, 1])
        );
    }
}
