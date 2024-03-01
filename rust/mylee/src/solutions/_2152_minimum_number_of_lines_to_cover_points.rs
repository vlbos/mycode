// # [2152. Minimum Number of Lines to Cover Points](https://leetcode.com/problems/minimum-number-of-lines-to-cover-points)

// ## Description

// You are given an array points where points[i] = [xi, yi] represents a point on an X-Y plane.

// Straight lines are going to be added to the X-Y plane, such that every point is covered by at least one line.

// Return the minimum number of straight lines needed to cover all the points.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2152.Minimum%20Number%20of%20Lines%20to%20Cover%20Points/images/image-20220123200023-1.png" style="width: 350px; height: 402px;" />
//
// Input: points = [[0,1],[2,3],[4,5],[4,3]]
// Output: 2
// Explanation: The minimum number of straight lines needed is two. One possible solution is to add:
// - One line connecting the point at (0, 1) to the point at (4, 5).
// - Another line connecting the point at (2, 3) to the point at (4, 3).
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2152.Minimum%20Number%20of%20Lines%20to%20Cover%20Points/images/image-20220123200057-3.png" style="width: 350px; height: 480px;" />
//
// Input: points = [[0,2],[-2,-2],[1,4]]
// Output: 1
// Explanation: The minimum number of straight lines needed is one. The only solution is to add:
// - One line connecting the point at (-2, -2) to the point at (1, 4).
//

// Constraints:

//
// 	1 <= points.length <= 10
// 	points[i].length == 2
// 	-100 <= xi, yi <= 100
// 	All the points are unique.
//

//   int minimum_lines(vector<vector<int>>& A) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_lines(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return 1;
        }
        let n1 = 1 << n;
        let mut dp = vec![i32::MAX / 3; n1];
        let same_line = |state: usize| {
            let mut temp = Vec::new();
            for i in 0..n {
                if state & (1 << i) > 0 {
                    temp.push(i);
                }
            }
            if temp.len() < 3 {
                return true;
            }
            let (x1, y1) = (points[temp[0]][0], points[temp[0]][1]);
            let (x2, y2) = (points[temp[1]][0], points[temp[1]][1]);
            for &t in &temp[2..] {
                let (curr_x, curr_y) = (points[t][0], points[t][1]);
                if (curr_x - x1) * (curr_y - y2) != (curr_x - x2) * (curr_y - y1) {
                    return false;
                }
            }
            true
        };
        for state in 1..n1 {
            if same_line(state) {
                dp[state] = 1;
            }
        }
        for state in 7..n1 {
            if dp[state] == 1 {
                continue;
            }
            let mut substate = state;
            while substate > 0 {
                dp[state] = dp[state].min(dp[substate] + dp[state - substate]);
                substate = state & (substate - 1);
            }
        }
        dp[n1 - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_lines_1() {
        assert_eq!(
            2,
            Solution::minimum_lines(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![4, 3]])
        );
    }
    #[test]
    pub fn test_minimum_lines_2() {
        assert_eq!(
            1,
            Solution::minimum_lines(vec![vec![0, 2], vec![-2, -2], vec![1, 4]])
        );
    }
}
