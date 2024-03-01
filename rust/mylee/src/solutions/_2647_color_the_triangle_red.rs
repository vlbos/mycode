// # [2647. Color the Triangle Red](https://leetcode.com/problems/color-the-triangle-red)

// ## Description

//  You are given an integer  n .
// Consider an equilateral triangle of side length  n , broken up into  n 2   unit equilateral triangles.
// The triangle has  n   1-indexed  rows where the  i th   row has  2i - 1  unit equilateral triangles.

//  The triangles in the  i th   row are also  1-indexed  with coordinates from  (i, 1)  to  (i, 2i - 1) .
// The following image shows a triangle of side length  4  with the indexing of its triangle.
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2647.Color%20the%20Triangle%20Red/images/triangle4.jpg" style="width: 402px; height: 242px;" />
//  Two triangles are  neighbors  if they  share a side .
// For example:

// 	 Triangles  (1,1)  and  (2,2)  are neighbors
// 	 Triangles  (3,2)  and  (3,3)  are neighbors.
// 	 Triangles  (2,2)  and  (3,3)  are not neighbors because they do not share any side.

//  Initially, all the unit triangles are  white .
// You want to choose  k  triangles and color them  red . We will then run the following algorithm:

// 	 Choose a white triangle that has  at least two  red neighbors.

//     	 If there is no such triangle, stop the algorithm.

//      Color that triangle  red .
//      Go to step 1.

//  Choose the minimum  k  possible and set  k  triangles red before running this algorithm such that after the algorithm stops,
// all unit triangles are colored red.

//  Return  a 2D list of the coordinates of the triangles that you will color red initially .
// The answer has to be of the smallest size possible. If there are multiple valid solutions, return any.

//   ### Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2647.Color%20the%20Triangle%20Red/images/example1.jpg" style="width: 500px; height: 263px;" />

//  Input:  n = 3
//  Output:  [[1,1],[2,1],[2,3],[3,1],[3,5]]
//  Explanation:  Initially, we choose the shown 5 triangles to be red. Then, we run the algorithm:
// - Choose (2,2) that has three red neighbors and color it red.
// - Choose (3,2) that has two red neighbors and color it red.
// - Choose (3,4) that has three red neighbors and color it red.
// - Choose (3,3) that has three red neighbors and color it red.
// It can be shown that choosing any 4 triangles and running the algorithm will not make all triangles red.

//   ### Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2647.Color%20the%20Triangle%20Red/images/example2.jpg" style="width: 300px; height: 101px;" />

//  Input:  n = 2
//  Output:  [[1,1],[2,1],[2,3]]
//  Explanation:  Initially, we choose the shown 3 triangles to be red. Then, we run the algorithm:
// - Choose (2,2) that has three red neighbors and color it red.
// It can be shown that choosing any 2 triangles and running the algorithm will not make all triangles red.

//   Constraints:

// 	  1  <= n  <= 1000

// ## Solutions

// **Approach 1: Find the Pattern**

// We draw a graph to observe, and we can find that the first row only has one triangle and must be colored,
// and from the last row to the second row, the coloring scheme of every four rows is the same:

// 1. The last row is colored at $(n, 1)$, $(n, 3)$, ..., $(n, 2n - 1)$.
// 1. The $n - 1$ row is colored at $(n - 1, 2)$.
// 1. The $n - 2$ row is colored at $(n - 2, 3)$, $(n - 2, 5)$, ..., $(n - 2, 2n - 5)$.
// 1. The $n - 3$ row is colored at $(n - 3, 1)$.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2647.Color%20the%20Triangle%20Red/images/demo3.png" style="width: 50%">

// Therefore, we can color the first row according to the above rules, and then start from the last row,
// and color every four rows once until the second row ends.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2600-2699/2647.Color%20the%20Triangle%20Red/images/demo2.png" style="width: 80%">

// The time complexity is $(n^2)$, where $n$ is the parameter given in the problem.
// Ignoring the space consumption of the answer array, the space complexity is $O(1)$.

// ### **C++**

// ```cpp
// class Solution {
// public:
//     vector<vector > color_red(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn color_red(n: i32) -> Vec<Vec<i32>> {
        // let mut ans = vec![vec![1, 1]];
        // let mut k = 0;
        // for i in (2..=n).rev() {
        //     match k {
        //         0 => {
        //             for j in (1..i << 1).step_by(2) {
        //                 ans.push(vec![i, j]);
        //             }
        //         }
        //         1 => ans.push(vec![i, 2]),
        //         2 => {
        //             for j in (3..i << 1).step_by(2) {
        //                 ans.push(vec![i, j]);
        //             }
        //         }
        //         _ => ans.push(vec![i, 1]),
        //     }
        //     k = (k + 1) % 4;
        // }
        // ans
        let mut ans = vec![];
        let tip_size = n % 4;
        if tip_size >= 1 {
            ans.push(vec![1, 1]);
        }
        for i in 2..=tip_size {
            ans.push(vec![i, 1]);
            ans.push(vec![i, 2 * i - 1]);
        }
        for i in (tip_size + 1..n).step_by(4) {
            ans.push(vec![i, 1]);
            for j in 1..=i {
                ans.push(vec![i + 1, 2 * j + 1]);
            }
            ans.push(vec![i + 2, 2]);
            for j in 0..=i + 2 {
                ans.push(vec![i + 3, 2 * j + 1]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_color_red_1() {
        assert_eq!(
            vec![vec![1, 1], vec![2, 1], vec![2, 3], vec![3, 1], vec![3, 5]],
            Solution::color_red(3)
        );
    }
    #[test]
    pub fn test_color_red_2() {
        assert_eq!(
            vec![vec![1, 1], vec![2, 1], vec![2, 3]],
            Solution::color_red(2)
        );
    }
}
