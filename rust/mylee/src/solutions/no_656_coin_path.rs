// 656\. Coin Path
// ===============

// Given an array `A` (index starts at `1`) consisting of N integers: A1, A2, ..., AN and an integer `B`.
// The integer `B` denotes that from any place (suppose the index is `i`) in the array `A`,
// you can jump to any one of the place in the array `A` indexed `i+1`, `i+2`, …, `i+B` if this place can be jumped to.
// Also, if you step on the index `i`, you have to pay Ai coins. If Ai is -1, it means you can’t jump to the place indexed `i` in the array.

// Now, you start from the place indexed `1` in the array `A`, and your aim is to reach the place indexed `N` using the minimum coins.
// You need to return the path of indexes (starting from 1 to N) in the array you should take to get to the place indexed `N` using minimum coins.

// If there are multiple paths with the same cost, return the lexicographically smallest such path.

// If it's not possible to reach the place indexed N then you need to return an empty array.

// **Example 1:**

// **Input:** \[1,2,4,-1,2\], 2
// **Output:** \[1,3,5\]

// **Example 2:**

// **Input:** \[1,2,4,-1,2\], 1
// **Output:** \[\]

// **Note:**

// 1.  Path Pa1, Pa2, ..., Pan is lexicographically smaller than Pb1, Pb2, ..., Pbm,
// if and only if at the first `i` where Pai and Pbi differ, Pai < Pbi; when no such `i` exists, then `n` < `m`.
// 2.  A1 \>= 0. A2, ..., AN (if exist) will in the range of \[-1, 100\].
// 3.  Length of A is in the range of \[1, 1000\].
// 4.  B is in the range of \[1, 100\].

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @star
// @lc code=start
impl Solution {
    pub fn cheapest_jump(a: Vec<i32>, b: i32) -> Vec<i32> {
        //     let n = a.len();
        //     if n == 0 || a[n - 1] < 0 {
        //         return vec![];
        //     } else if n == 1 {
        //         return vec![1];
        //     }
        //     let mut costs = vec![i64::max_value(); n];
        //     costs[n - 1] = 0;
        //     let mut next = vec![-1; n];
        //     for y in (0..(n - 1)).rev() {
        //         let x_start = y + 1;
        //         let x_end = i32::min((n - 1) as i32, (y as i32) + b) as usize;
        //         let mut min_cost = i64::max_value();
        //         let mut min_to = -1;
        //         for x in x_start..=x_end {
        //             let cost = if a[y] < 0 || costs[x] == i64::max_value() {
        //                 i64::max_value()
        //             } else {
        //                 costs[x] + a[y] as i64
        //             };
        //             if cost < min_cost {
        //                 min_cost = cost;
        //                 min_to = x as i32;
        //             }
        //         }
        //         costs[y] = min_cost;
        //         next[y] = min_to;
        //     }
        //     if costs[0] == i64::max_value() {
        //         vec![]
        //     } else {
        //         let mut path = vec![0];
        //         let mut curr = 0i32;
        //         while curr != (n - 1) as i32 {
        //             curr = next[curr as usize];
        //             path.push(curr);
        //         }
        //         path.into_iter().map(|v| v + 1).collect::<Vec<_>>()
        //     }
        let n = a.len();
        if a[n - 1] == -1 {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let mut dp = vec![i32::MAX; n];
        let mut pos = vec![n; n];
        dp[n - 1] = a[n - 1];
        for i in (0..n - 1).rev() {
            if a[i] == -1 {
                continue;
            }
            for j in i + 1..=(n - 1).min(i + b as usize) {
                if dp[j] == i32::MAX {
                    continue;
                }
                if a[i] + dp[j] < dp[i] {
                    dp[i] = a[i] + dp[j];
                    pos[i] = j;
                }
            }
        }
        if dp[0] == i32::MAX {
            return ans;
        }
        let mut cur = 0;
        while cur != n {
            ans.push(cur as i32 + 1);
            cur = pos[cur];
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coin_path_1() {
        assert_eq!(
            Solution::cheapest_jump(vec![1, 2, 4, -1, 2], 2),
            vec![1, 3, 5]
        );
    }

    #[test]
    fn test_coin_path_2() {
        assert_eq!(Solution::cheapest_jump(vec![1, 2, 4, -1, 2], 1), vec![]);
    }

    #[test]
    fn test_coin_path_3() {
        assert_eq!(
            Solution::cheapest_jump(vec![0, 0, 0, 0, 0, 0], 3),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_coin_path_4() {
        assert_eq!(
            Solution::cheapest_jump(vec![0, -1, -1, -1, -1, -1], 5),
            vec![]
        );
    }
}
