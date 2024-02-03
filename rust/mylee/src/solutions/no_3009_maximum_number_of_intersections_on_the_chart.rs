// # [3009. Maximum Number of Intersections on the Chart](https://leetcode.com/problems/maximum-number-of-intersections-on-the-chart)

// ## Description

// There is a line chart consisting of n points connected by line segments. You are given a 1-indexed integer array y. The kth point has coordinates (k, y[k]). There are no horizontal lines; that is, no two consecutive points have the same y-coordinate.

// We can draw an infinitely long horizontal line. Return the maximum number of points of intersection of the line with the chart.

//
// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3000-3099/3009.Maximum%20Number%20of%20Intersections%20on%20the%20Chart/images/20231208-020549.jpeg" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; height: 217px; width: 600px;" />

// Input: y = [1,2,1,2,1,3,2]
// Output: 5
// Explanation: As you can see in the image above, the line y = 1.5 has 5 intersections with the chart (in red crosses). You can also see the line y = 2 which intersects the chart in 4 points (in red crosses). It can be shown that there is no horizontal line intersecting the chart at more than 5 points. So the answer would be 5.

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3000-3099/3009.Maximum%20Number%20of%20Intersections%20on%20the%20Chart/images/20231208-020557.jpeg" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 400px; height: 404px;" />

// Input: y = [2,1,3,4,5]
// Output: 2
// Explanation: As you can see in the image above, the line y = 1.5 has 2 intersections with the chart (in red crosses). You can also see the line y = 2 which intersects the chart in 2 points (in red crosses). It can be shown that there is no horizontal line intersecting the chart at more than 2 points. So the answer would be 2.

//
// Constraints:

// 	2  <= y.length  <= 105
// 	1  <= y[i]  <= 109
// 	y[i] != y[i + 1] for i in range [1, n - 1]

// class Solution {
//  public:
//   int max_intersection_count(vector<int>& y) {
//     const int n = y.size();
//     int ans = 0;
//     int intersectionCount = 0;
//     map<int, int> line;

//     for (int i = 1; i < n; ++i) {
//       const int start = 2 * y[i - 1];
//       const int end = 2 * y[i] + (i == n - 1 ? 0 : y[i] > y[i - 1] ? -1 : 1);
//       ++line[min(start, end)];
//       --line[max(start, end) + 1];
//     }

//     for (const auto& [_, count] : line) {
//       intersectionCount += count;
//       ans = max(ans, intersectionCount);
//     }

//     return ans;
//   }
// };

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_intersection_count(y: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_intersection_count_1() {
        assert_eq!(
            5,
            Solution::max_intersection_count(vec![1, 2, 1, 2, 1, 3, 2])
        );
    }
    #[test]
    pub fn test_max_intersection_count_2() {
        assert_eq!(2, Solution::max_intersection_count(vec![2, 1, 3, 4, 5]));
    }
}
