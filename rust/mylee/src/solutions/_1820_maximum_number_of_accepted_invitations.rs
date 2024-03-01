// 1820\. Maximum Number of Accepted Invitations
// =============================================

// There are `m` boys and `n` girls in a class attending an upcoming party.

// You are given an `m x n` integer matrix `grid`, where `grid[i][j]` equals `0` or `1`. If `grid[i][j] == 1`,
// then that means the `ith` boy can invite the `jth` girl to the party.
// A boy can invite at most **one girl**, and a girl can accept at most **one invitation** from a boy.

// Return _the **maximum** possible number of accepted invitations._

// **Example 1:**

// **Input:** grid = \[\[1,1,1\],
//                \[1,0,1\],
//                \[0,0,1\]\]
// **Output:** 3 **Explanation:** The invitations are sent as follows:
// - The 1st boy invites the 2nd girl.
// - The 2nd boy invites the 1st girl.
// - The 3rd boy invites the 3rd girl.

// **Example 2:**

// **Input:** grid = \[\[1,0,1,0\],
//                \[1,0,0,0\],
//                \[0,0,1,0\],
//                \[1,1,1,0\]\]
// **Output:** 3
// **Explanation:** The invitations are sent as follows:
// -The 1st boy invites the 3rd girl.
// -The 2nd boy invites the 1st girl.
// -The 3rd boy invites no one.
// -The 4th boy invites the 2nd girl.

// **Constraints:**

// *   `grid.length == m`
// *   `grid[i].length == n`
// *   `1 <= m, n <= 200`
// *   `grid[i][j]` is either `0` or `1`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

//    int maximum_invitations(vector<vector<int>>& grid)

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn maximum_invitations(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut mate = vec![-1; n];
        let mut ans = 0;
        fn can_invite(
            i: usize,
            grid: &Vec<Vec<i32>>,
            seen: &mut Vec<bool>,
            mate: &mut Vec<i32>,
        ) -> bool {
            for j in 0..seen.len() {
                if grid[i][j] == 0 || seen[j] {
                    continue;
                }
                seen[j] = true;
                if mate[j] == -1 || can_invite(mate[j] as usize, grid, seen, mate) {
                    mate[j] = i as i32;
                    return true;
                }
            }
            false
        }
        for i in 0..m {
            let mut seen = vec![false; n];
            if can_invite(i, &grid, &mut seen, &mut mate) {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_invitations_1() {
        assert_eq!(
            3,
            Solution::maximum_invitations(vec![vec![1, 1, 1], vec![1, 0, 1], vec![0, 0, 1]])
        );
    }
    #[test]
    pub fn test_maximum_invitations_2() {
        assert_eq!(
            3,
            Solution::maximum_invitations(vec![
                vec![1, 0, 1, 0],
                vec![1, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![1, 1, 1, 0]
            ])
        );
    }
}
