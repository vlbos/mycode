// # [3189. Minimum Moves to Get a Peaceful Board 🔒](https://leetcode.com/problems/minimum-moves-to-get-a-peaceful-board)

// ## Description

//

// Given a 2D array rooks of length n,
// where rooks[i] = [xi, yi] indicates the position of a rook on an n x n chess board.
//  Your task is to move the rooks 1 cell at a time vertically or horizontally (to an adjacent cell) such that the board becomes peaceful.

// A board is peaceful if there is exactly one rook in each row and each column.

// Return the minimum number of moves required to get a peaceful board.

// Note that at no point can there be two rooks in the same cell.

//
// Example 1:

//
// Input: rooks = [[0,0],[1,0],[1,1]]

// Output: 3

// Explanation:
//

// Example 2:

//
// Input: rooks = [[0,0],[0,1],[0,2],[0,3]]

// Output: 6

// Explanation:
//

//
// Constraints:

//
// 	1 <= n == rooks.length <= 500
// 	0 <= xi, yi <= n - 1
// 	The input is generated such that there are no 2 rooks in the same cell.
//

//     int min_moves(vector<vector<int>>& rooks) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_moves(mut rooks: Vec<Vec<i32>>) -> i32 {
        rooks.sort_unstable();
        let mut ans = rooks
            .iter()
            .enumerate()
            .map(|(i, x)| x[0].abs_diff(i as i32))
            .sum::<u32>();
        rooks.sort_unstable_by_key(|x| x[1]);
        ans += rooks
            .iter()
            .enumerate()
            .map(|(i, x)| x[1].abs_diff(i as i32))
            .sum::<u32>();
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_min_moves_1() {
        assert_eq!(3, Solution::min_moves(lc_matrix![[0, 0], [1, 0], [1, 1]]));
    }
    #[test]
    pub fn test_min_moves_2() {
        assert_eq!(
            6,
            Solution::min_moves(lc_matrix![[0, 0], [0, 1], [0, 2], [0, 3]])
        );
    }
}
