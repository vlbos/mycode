// # [2604. Minimum Time to Eat All Grains](https://leetcode.com/problems/minimum-time-to-eat-all-grains)

// ## Description

//  There are  n  hens and  m  grains on a line.
// You are given the initial positions of the hens and the grains in two integer arrays  hens  and  grains  of size  n  and  m  respectively.

//  Any hen can eat a grain if they are on the same position.
// The time taken for this is negligible. One hen can also eat multiple grains.
//  In  1  second, a hen can move right or left by  1  unit.
// The hens can move simultaneously and independently of each other.
//  Return  the  minimum  time to eat all grains if the hens act optimally.

//   ### Example 1:

//  Input:  hens = [3,6,7], grains = [2,4,7,9]
//  Output:  2
//  Explanation:
// One of the ways hens eat all grains in 2 seconds is described below:
// - The first hen eats the grain at position 2 in 1 second.
// - The second hen eats the grain at position 4 in 2 seconds.
// - The third hen eats the grains at positions 7 and 9 in 2 seconds.
// So, the maximum time needed is 2.
// It can be proven that the hens cannot eat all grains before 2 seconds.

//   ### Example 2:

//  Input:  hens = [4,6,109,111,213,215], grains = [5,110,214]
//  Output:  1
//  Explanation:
// One of the ways hens eat all grains in 1 second is described below:
// - The first hen eats the grain at position 5 in 1 second.
// - The fourth hen eats the grain at position 110 in 1 second.
// - The sixth hen eats the grain at position 214 in 1 second.
// - The other hens do not move.
// So, the maximum time needed is 1.

//   Constraints:

// 	  1  <= hens.length, grains.length  <= 2*10^ 4
// 	  0  <= hens[i], grains[j]  <= 10^9

// ## Solutions

// **Approach 1: Sorting + Binary Search**

// First, sort the chickens and grains by their position from left to right.
// Then enumerate the time $t$ using binary search to find the smallest $t$ such that all the grains can be eaten up in $t$ seconds.

// For each chicken, we use the pointer $j$ to point to the leftmost grain that has not been eaten,
// and the current position of the chicken is $x$ and the position of the grain is $y$. There are the following cases:

// -   If $y <= x$, we note that $d = x - y$. If $d > t$, the current grain cannot be eaten, so directly return `false`.
// Otherwise, move the pointer $j$ to the right until $j=m$ or $grains[j] > x$.
// At this point, we need to check whether the chicken can eat the grain pointed to by $j$.
// If it can, continue to move the pointer $j$ to the right until $j=m$ or $min(d, grains[j] - x) + grains[j] - y > t$.
// -   If $y < x$, move the pointer $j$ to the right until $j=m$ or $grains[j] - x > t$.

// If $j=m$, it means that all the grains have been eaten, return `true`, otherwise return `false`.

// Time complexity $O(n * \log n + m * \log m + (m + n) * \log U)$,
// space complexity $O(\log m + \log n)$. $n$ and $m$ are the number of chickens and grains respectively,
// and $U$ is the maximum value of all the chicken and grain positions.

// ### **C++**

// ```cpp
// class Solution {
// public:
//     int minimum_time(vector & hens, vector & grains) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn minimum_time(mut hens: Vec<i32>, mut grains: Vec<i32>) -> i32 {
        hens.sort_unstable();
        grains.sort_unstable();
        let m = grains.len();
        let check = |t: i32| {
            let mut j = 0;
            for &x in &hens {
                if j == m {
                    return true;
                }
                let y = grains[j];
                if y <= x {
                    let d = x - y;
                    if d > t {
                        return false;
                    }
                    while j < m && grains[j] <= x {
                        j += 1;
                    }
                    while j < m && d.min(grains[j] - x) + grains[j] - y <= t {
                        j += 1;
                    }
                } else {
                    while j < m && grains[j] - x <= t {
                        j += 1;
                    }
                }
            }
            j == m
        };
        let (mut l, mut r) = (
            0,
            (hens[0] - grains[0]).abs() + grains[m - 1] - grains[0] + 1,
        );
        while l < r {
            let mid = (l + r) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_time_1() {
        assert_eq!(2, Solution::minimum_time(vec![3, 6, 7], vec![2, 4, 7, 9]));
    }
    #[test]
    pub fn test_minimum_time_2() {
        assert_eq!(
            1,
            Solution::minimum_time(vec![4, 6, 109, 111, 213, 215], vec![5, 110, 214])
        );
    }
}
