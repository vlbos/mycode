// # [2184. Number of Ways to Build Sturdy Brick Wall](https://leetcode.com/problems/number-of-ways-to-build-sturdy-brick-wall)

// ## Description

// You are given integers height and width which specify the dimensions of a brick wall you are building.
//  You are also given a 0-indexed array of unique integers bricks, where the ith brick has a height of 1 and a width of bricks[i].
// You have an infinite supply of each type of brick and bricks may not be rotated.

// Each row in the wall must be exactly width units long. For the wall to be sturdy,
// adjacent rows in the wall should not join bricks at the same location, except at the ends of the wall.

// Return the number of ways to build a sturdy wall. Since the answer may be very large, return it modulo 109 + 7.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2184.Number%20of%20Ways%20to%20Build%20Sturdy%20Brick%20Wall/images/image-20220220190749-1.png" style="width: 919px; height: 250px;" />
//
// Input: height = 2, width = 3, bricks = [1,2]
// Output: 2
// Explanation:
// The first two walls in the diagram show the only two ways to build a sturdy brick wall.
// Note that the third wall in the diagram is not sturdy because adjacent rows join bricks 2 units from the left.
//

// Example 2:

//
// Input: height = 1, width = 1, bricks = [5]
// Output: 0
// Explanation:
// There are no ways to build a sturdy wall because the only type of brick we have is longer than the width of the wall.
//

// Constraints:

//
// 	1 <= height <= 100
// 	1 <= width <= 10
// 	1 <= bricks.length <= 10
// 	1 <= bricks[i] <= 10
// 	All the values of bricks are unique.
//
// int build_wall(int height, int width, vector<int>& bricks) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn build_wall(height: i32, width: i32, bricks: Vec<i32>) -> i32 {
        let mut rows = Vec::new();
        fn build_rows(width: i32, bricks: &Vec<i32>, path: i32, rows: &mut Vec<i32>) {
            for &brick in bricks {
                if brick == width {
                    rows.push(path);
                } else if brick < width {
                    let new_width = width - brick;
                    build_rows(new_width, bricks, path | (1 << new_width), rows);
                }
            }
        }
        build_rows(width, &bricks, 0, &mut rows);
        let n = rows.len();
        let mut dp = vec![1; n];
        let mut graph = vec![Vec::new(); n];
        for i in 0..n {
            for j in 0..n {
                if rows[i] & rows[j] == 0 {
                    graph[i].push(j);
                }
            }
        }
        for h in 2..=height {
            let mut new_dp = vec![0; n];
            for i in 0..n {
                for &v in &graph[i] {
                    new_dp[i] += dp[v];
                    new_dp[i] %= 1_000_000_007;
                }
            }
            dp = new_dp;
        }
        dp.into_iter().sum::<i32>() % 1_000_000_007
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_build_wall_1() {
        assert_eq!(2, Solution::build_wall(2, 3, vec![1, 2]));
    }
    #[test]
    pub fn test_build_wall_2() {
        assert_eq!(0, Solution::build_wall(1, 1, vec![5]));
    }
}
