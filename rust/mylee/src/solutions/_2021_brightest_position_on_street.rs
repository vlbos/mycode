// [2021\. Brightest Position on Street (Medium)](https://leetcode.com/problems/brightest-position-on-street/)[](https://leetcode.ca/2021-10-05-2021-Brightest-Position-on-Street/#2021-brightest-position-on-street-medium)
// =========================================================================================================================================================================================================================

// A perfectly straight street is represented by a number line.
// The street has street lamp(s) on it and is represented by a 2D integer array `lights`.
// Each `lights[i] = [positioni, rangei]` indicates that there is a street lamp at position `positioni` that lights up the area from `[positioni - rangei, positioni + rangei]` (**inclusive**).

// The **brightness** of a position `p` is defined as the number of street lamp that light up the position `p`.

// Given `lights`, return _the **brightest** position on the_ _street. If there are multiple brightest positions, return the **smallest** one._

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2021/09/28/image-20210928155140-1.png)

// **Input:** lights = \[\[-3,2\],\[1,2\],\[3,3\]\]
// **Output:** -1
// **Explanation:**
// The first street lamp lights up the area from \[(-3) - 2, (-3) + 2\] = \[-5, -1\].
// The second street lamp lights up the area from \[1 - 2, 1 + 2\] = \[-1, 3\].
// The third street lamp lights up the area from \[3 - 3, 3 + 3\] = \[0, 6\].

// Position -1 has a brightness of 2, illuminated by the first and second street light.
// Positions 0, 1, 2, and 3 have a brightness of 2, illuminated by the second and third street light.
// Out of all these positions, -1 is the smallest, so return it.

// **Example 2:**

// **Input:** lights = \[\[1,0\],\[0,1\]\]
// **Output:** 1
// **Explanation:**
// The first street lamp lights up the area from \[1 - 0, 1 + 0\] = \[1, 1\].
// The second street lamp lights up the area from \[0 - 1, 0 + 1\] = \[-1, 1\].

// Position 1 has a brightness of 2, illuminated by the first and second street light.
// Return 1 because it is the brightest position on the street.

// **Example 3:**

// **Input:** lights = \[\[1,2\]\]
// **Output:** -1
// **Explanation:**
// The first street lamp lights up the area from \[1 - 2, 1 + 2\] = \[-1, 3\].

// Positions -1, 0, 1, 2, and 3 have a brightness of 1, illuminated by the first street light.
// Out of all these positions, -1 is the smallest, so return it.

// **Constraints:**

// *   `1 <= lights.length <= 105`
// *   `lights[i].length == 2`
// *   `-108 <= positioni <= 108`
// *   `0 <= rangei <= 108`

// **Companies**:
// [Amazon](https://leetcode.com/company/amazon)

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/), [Prefix Sum](https://leetcode.com/tag/prefix-sum/), [Ordered Set](https://leetcode.com/tag/ordered-set/)

// **Similar Questions**:

// *   [Minimum Number of Buckets Required to Collect Rainwater from Houses (Medium)](https://leetcode.com/problems/minimum-number-of-buckets-required-to-collect-rainwater-from-houses/)

// Solution 1. Heap[](https://leetcode.ca/2021-10-05-2021-Brightest-Position-on-Street/#solution-1-heap)
// -----------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/brightest-position-on-street/
//     // Time: O(NlogN)
//     // Space: O(N)
//     class Solution {
//     public:
//         int brightest_position(vector<vector<int>>& A) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn brightest_position(lights: Vec<Vec<i32>>) -> i32 {
        let mut a: Vec<(i32, i32)> = lights
            .into_iter()
            .map(|v| (v[0] - v[1], v[0] + v[1]))
            .collect();
        a.sort();
        let mut q = std::collections::BinaryHeap::<Reverse<i32>>::new();
        use std::cmp::Reverse;
        let mut ans = i32::MIN;
        let mut len = 0;
        for &(begin, end) in &a {
            while !q.is_empty() && q.peek().unwrap().0 < begin {
                q.pop();
            }
            q.push(Reverse(end));
            if q.len() > len {
                len = q.len();
                ans = begin;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_brightest_position_1() {
        assert_eq!(
            -1,
            Solution::brightest_position(vec![vec![-3, 2], vec![1, 2], vec![3, 3]])
        );
    }
    #[test]
    pub fn test_brightest_position_2() {
        assert_eq!(
            1,
            Solution::brightest_position(vec![vec![1, 0], vec![0, 1]])
        );
    }
    #[test]
    pub fn test_brightest_position_3() {
        assert_eq!(-1, Solution::brightest_position(vec![vec![1, 2]]));
    }
}
