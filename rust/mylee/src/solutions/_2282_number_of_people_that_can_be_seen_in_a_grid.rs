// # [2282. Number of People That Can Be Seen in a Grid](https://leetcode.com/problems/number-of-people-that-can-be-seen-in-a-grid)

// ## Description

// You are given an m x n 0-indexed 2D array of positive integers heights where heights[i][j] is the height of the person standing at position (i, j).

// A person standing at position (row1, col1) can see a person standing at position (row2, col2) if:

//
// 	The person at (row2, col2) is to the right or below the person at (row1, col1). More formally, this means that either row1 == row2 and col1 < col2 or row1 < row2 and col1 == col2.
// 	Everyone in between them is shorter than both of them.
//

// Return an m x n 2D array of integers answer where answer[i][j] is the number of people that the person at position (i, j) can see.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2282.Number%20of%20People%20That%20Can%20Be%20Seen%20in%20a%20Grid/images/image-20220524180458-1.png" style="width: 700px; height: 164px;" />
//
// Input: heights = [[3,1,4,2,5]]
// Output: [[2,1,2,1,0]]
// Explanation:
// - The person at (0, 0) can see the people at (0, 1) and (0, 2).
//   Note that he cannot see the person at (0, 4) because the person at (0, 2) is taller than him.
// - The person at (0, 1) can see the person at (0, 2).
// - The person at (0, 2) can see the people at (0, 3) and (0, 4).
// - The person at (0, 3) can see the person at (0, 4).
// - The person at (0, 4) cannot see anybody.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2282.Number%20of%20People%20That%20Can%20Be%20Seen%20in%20a%20Grid/images/image-20220523113533-2.png" style="width: 400px; height: 249px;" />
//
// Input: heights = [[5,1],[3,1],[4,1]]
// Output: [[3,1],[2,1],[1,0]]
// Explanation:
// - The person at (0, 0) can see the people at (0, 1), (1, 0) and (2, 0).
// - The person at (0, 1) can see the person at (1, 1).
// - The person at (1, 0) can see the people at (1, 1) and (2, 0).
// - The person at (1, 1) can see the person at (2, 1).
// - The person at (2, 0) can see the person at (2, 1).
// - The person at (2, 1) cannot see anybody.
//

// Constraints:

//
// 	1 <= heights.length <= 400
// 	1 <= heights[i].length <= 400
// 	1 <= heights[i][j] <= 10^5
//

// vector<vector<int>> see_people(vector<vector<int>>& A)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn see_people(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        let mut ans = vec![vec![0; n]; m];
        let mut s = Vec::new();
        for (i, row) in heights.iter().enumerate() {
            s.clear();
            for (j, &v) in row.iter().enumerate().rev() {
                let mut seen = 0;
                while !s.is_empty() && *s.last().unwrap() < v {
                    seen += 1;
                    s.pop();
                }
                ans[i][j] += seen + if !s.is_empty() { 1 } else { 0 };
                if s.is_empty() || *s.last().unwrap() != v {
                    s.push(v);
                }
            }
        }
        for j in 0..n {
            s.clear();
            for i in (0..m).rev() {
                let mut seen = 0;
                while !s.is_empty() && *s.last().unwrap() < heights[i][j] {
                    seen += 1;
                    s.pop();
                }
                ans[i][j] += seen + if !s.is_empty() { 1 } else { 0 };
                if s.is_empty() || *s.last().unwrap() != heights[i][j] {
                    s.push(heights[i][j]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_see_people_1() {
        assert_eq!(
            vec![vec![2, 1, 2, 1, 0]],
            Solution::see_people(vec![vec![3, 1, 4, 2, 5]])
        );
    }
    #[test]
    pub fn test_see_people_2() {
        assert_eq!(
            vec![vec![3, 1], vec![2, 1], vec![1, 0]],
            Solution::see_people(vec![vec![5, 1], vec![3, 1], vec![4, 1]])
        );
    }
}
