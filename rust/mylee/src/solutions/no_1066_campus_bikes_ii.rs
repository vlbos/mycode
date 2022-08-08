// 1066\. Campus Bikes II
// ======================

// On a campus represented as a 2D grid, there are `N` workers and `M` bikes, with `N <= M`. Each worker and bike is a 2D coordinate on this grid.

// We assign one unique bike to each worker so that the sum of the Manhattan distances between each worker and their assigned bike is minimized.

// The Manhattan distance between two points `p1` and `p2` is `Manhattan(p1, p2) = |p1.x - p2.x| + |p1.y - p2.y|`.

// Return the minimum possible sum of Manhattan distances between each worker and their assigned bike.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2019/03/06/1261_example_1_v2.png)

// **Input:** workers = \[\[0,0\],\[2,1\]\], bikes = \[\[1,2\],\[3,3\]\]
// **Output:** 6
// **Explanation:**
// We assign bike 0 to worker 0, bike 1 to worker 1. The Manhattan distance of both assignments is 3, so the output is 6.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2019/03/06/1261_example_2_v2.png)

// **Input:** workers = \[\[0,0\],\[1,1\],\[2,0\]\], bikes = \[\[1,0\],\[2,2\],\[2,1\]\]
// **Output:** 4
// **Explanation:**
// We first assign bike 0 to worker 0, then assign bike 1 to worker 1 or worker 2, bike 2 to worker 2 or worker 1. Both assignments lead to sum of the Manhattan distances as 4.

// **Note:**

// 1.  `0 <= workers[i][0], workers[i][1], bikes[i][0], bikes[i][1] < 1000`
// 2.  All worker and bike locations are distinct.
// 3.  `1 <= workers.length <= bikes.length <= 10`
// **Constraints:**

// *   `n == workers.length`
// *   `m == bikes.length`
// *   `1 <= n <= m <= 10`
// *   `workers[i].length == 2`
// *   `bikes[i].length == 2`
// *   `0 <= workers[i][0], workers[i][1], bikes[i][0], bikes[i][1] < 1000`
// *   All the workers and the bikes locations are **unique**.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

pub struct Solution {}
impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (workers.len(), bikes.len());
        let mut ans = i32::MAX;
        let mut dp = vec![i32::MAX / 3; 1 << m];
        dp[0] = 0;
        for mask in 0..(1 << m) as usize {
            let worker_idx = mask.count_ones() as usize;
            if worker_idx >= n {
                ans = ans.min(dp[mask]);
                continue;
            }
            for bike_idx in 0..m {
                if (mask & (1 << bike_idx)) == 0 {
                    let dist = (workers[worker_idx][0] - bikes[bike_idx][0]).abs()
                        + (workers[worker_idx][1] - bikes[bike_idx][1]).abs();
                    let new_mask = (mask | (1 << bike_idx));
                    dp[new_mask] = dp[new_mask].min(dist + dp[mask]);
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
    fn test_assign_bikes_1() {
        assert_eq!(
            6,
            Solution::assign_bikes(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]])
        );
    }

    #[test]
    fn test_assign_bikes_2() {
        assert_eq!(
            4,
            Solution::assign_bikes(
                vec![vec![0, 0], vec![1, 1], vec![2, 0]],
                vec![vec![1, 0], vec![2, 2], vec![2, 1]]
            )
        );
    }
    #[test]
    fn test_assign_bikes_3() {
        assert_eq!(
            4995,
            Solution::assign_bikes(
                vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0]],
                vec![
                    vec![0, 999],
                    vec![1, 999],
                    vec![2, 999],
                    vec![3, 999],
                    vec![4, 999]
                ]
            )
        );
    }
}
