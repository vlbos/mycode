// 351\. Android Unlock Patterns
// =============================

// Given an Android **3x3** key lock screen and two integers **m** and **n**,
// where 1 ≤ m ≤ n ≤ 9, count the total number of unlock patterns of the Android lock screen,
// which consist of minimum of **m** keys and maximum **n** keys.

// **Rules for a valid pattern:**

// 1.  Each pattern must connect at least **m** keys and at most **n** keys.
// 2.  All the keys must be distinct.
// 3.  If the line connecting two consecutive keys in the pattern passes through any other keys,
// the other keys must have previously selected in the pattern.
// No jumps through non selected key is allowed.
// 4.  The order of keys used matters.

// ![](https://assets.leetcode.com/uploads/2018/10/12/android-unlock.png)

// **Explanation:**

// | 1 | 2 | 3 |
// | 4 | 5 | 6 |
// | 7 | 8 | 9 |

// **Invalid move:** `4 - 1 - 3 - 6`
// Line 1 - 3 passes through key 2 which had not been selected in the pattern.

// **Invalid move:** `4 - 1 - 9 - 2`
// Line 1 - 9 passes through key 5 which had not been selected in the pattern.

// **Valid move:** `2 - 4 - 1 - 3 - 6`
// Line 1 - 3 is valid because it passes through key 2, which had been selected in the pattern

// **Valid move:** `6 - 5 - 4 - 1 - 9 - 2`
// Line 1 - 9 is valid because it passes through key 5, which had been selected in the pattern.

// **Example:**

// **Input:** m = 1, n = 1
// **Output:** 9

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::{HashMap, HashSet};

// static mut INDIRECT: Vec<HashMap<i32, i32>> = vec![];

// fn try_init_indirect() -> &'static Vec<HashMap<i32, i32>> {
//     unsafe {
//         if INDIRECT.is_empty() {
//             INDIRECT = vec![
//                 vec![],
//                 vec![(2, 3), (4, 7), (5, 9)],
//                 vec![(5, 8)],
//                 vec![(2, 1), (6, 9), (5, 7)],
//                 vec![(5, 6)],
//                 vec![],
//                 vec![(5, 4)],
//                 vec![(4, 1), (8, 9), (5, 3)],
//                 vec![(5, 2)],
//                 vec![(6, 3), (8, 7), (5, 1)],
//             ]
//             .into_iter()
//             .map(|v| {
//                 v.into_iter()
//                     .map(|(k, v)| (v, k))
//                     .collect::<HashMap<i32, i32>>()
//             })
//             .collect();
//         }
//         &INDIRECT
//     }
// }

// #[derive(Debug)]
// struct Route {
//     used: HashSet<i32>,
//     curr: i32,
//     unused: HashSet<i32>,
// }

// impl Route {
//     pub fn new() -> Self {
//         // Route {
//         //     used: HashSet::new(),
//         //     curr: 0,
//         //     unused: (1..=9).collect::<HashSet<_>>(),
//         // }
//     }

//     pub fn accessible(&self) -> Vec<Route> {
//         let indirect = &try_init_indirect()[self.curr as usize];
//         self.unused
//             .iter()
//             .map(|i| {
//                 if indirect.contains_key(&i) {
//                     let passthrough = indirect[&i];
//                     if self.used.contains(&passthrough) {
//                         Some(*i)
//                     } else {
//                         None
//                     }
//                 } else {
//                     Some(*i)
//                 }
//             })
//             .filter(|s| s.is_some())
//             .map(|target| self.next(target.unwrap()))
//             .collect()
//     }

//     pub fn next(&self, target: i32) -> Self {
//         let mut next_used = self.used.clone();
//         next_used.insert(target);
//         let mut next_unused = self.unused.clone();
//         next_unused.remove(&target);
//         return Route {
//             used: next_used,
//             curr: target,
//             unused: next_unused,
//         };
//     }
// }

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        // let mut rs = vec![Route::new()];
        // let mut count = 0;
        // while let Some(r) = rs.pop() {
        //     if r.used.len() as i32 >= m && r.used.len() as i32 <= n {
        //         count += 1;
        //     }
        //     if (r.used.len() as i32) < n {
        //         rs.extend(r.accessible());
        //     }
        // }
        // count
        let states = 1 << 9;
        let mut dp = vec![vec![0; 10]; states];
        let mut skip = vec![vec![0; 10]; 10];
        for &(i, j, k) in &[(1, 3, 2), (1, 7, 4), (3, 9, 6), (7, 9, 8)] {
            skip[i][j] = k;
            skip[j][i] = k;
        }
        for i in 1..=4 {
            let j = 10 - i;
            skip[i][j] = 5;
            skip[j][i] = 5;
        }
        for i in 1..10 {
            dp[1 << (i - 1)][i] = 1;
        }
        for state in 0..states {
            for end in 1..10 {
                if dp[state][end] == 0 {
                    continue;
                }
                for new_end in 1..10 {
                    if state & (1 << (new_end - 1)) > 0 {
                        continue;
                    }
                    if skip[end][new_end] == 0 || ((state & (1 << (skip[end][new_end] - 1))) > 0) {
                        dp[state | (1 << (new_end - 1))][new_end] += dp[state][end];
                    }
                }
            }
        }
        let mut ans = 0;
        for state in 0..states {
            if state.count_ones() >= m as u32 && state.count_ones() <= n as u32 {
                for end in 1..10 {
                    ans += dp[state][end];
                }
            }
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
    fn test_number_of_patterns_1() {
        assert_eq!(Solution::number_of_patterns(1, 1), 9);
    }

    #[test]
    fn test_number_of_patterns_2() {
        assert_eq!(Solution::number_of_patterns(1, 2), 65);
    }
}
