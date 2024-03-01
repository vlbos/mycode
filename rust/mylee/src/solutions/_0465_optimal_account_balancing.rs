// 465\. Optimal Account Balancing
// ===============================

// A group of friends went on holiday and sometimes lent each other money. For example,
// Alice paid for Bill's lunch for $10. Then later Chris gave Alice $5 for a taxi ride.
// We can model each transaction as a tuple (x, y, z) which means person x gave person y $z.
// Assuming Alice, Bill, and Chris are person 0, 1, and 2 respectively (0, 1, 2 are the person's ID),
// the transactions can be represented as `[[0, 1, 10], [2, 0, 5]]`.

// Given a list of transactions between a group of people, return the minimum number of transactions required to settle the debt.

// **Note:**

// 1.  A transaction will be given as a tuple (x, y, z). Note that `x ≠ y` and `z > 0`.
// 2.  Person's IDs may not be linear, e.g. we could have the persons 0, 1, 2 or we could also have the persons 0, 2, 6.

// **Example 1:**

// **Input:**
// \[\[0,1,10\], \[2,0,5\]\]

// **Output:**
// 2

// **Explanation:**
// Person #0 gave person #1 $10.
// Person #2 gave person #0 $5.

// Two transactions are needed. One way to settle the debt is person #1 pays person #0 and #2 $5 each.

// **Example 2:**

// **Input:**
// \[\[0,1,10\], \[1,0,1\], \[1,2,5\], \[2,0,5\]\]

// **Output:**
// 1

// **Explanation:**
// Person #0 gave person #1 $10.
// Person #1 gave person #0 $1.
// Person #1 gave person #2 $5.
// Person #2 gave person #0 $5.

// Therefore, person #1 only need to give person #0 $4, and all debt is settled.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// use std::collections::HashMap;

impl Solution {
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        // let mut balance = HashMap::<i32, i32>::new();
        // for v in transactions {
        //     let i = v[0];
        //     let o = v[1];
        //     let a = v[2];
        //     balance.entry(i).and_modify(|b| *b += a).or_insert(a);
        //     balance.entry(o).and_modify(|b| *b -= a).or_insert(0 - a);
        // }
        // let employees = balance
        //     .into_iter()
        //     .filter(|(_, account)| *account != 0)
        //     .map(|(_, account)| account)
        //     .collect::<Vec<i32>>();
        // let len = employees.len() as i32;
        // let size = 1usize << len;
        // let mut sum = vec![0; size];
        // for i in 0..(1 << len) {
        //     for j in 0..len {
        //         if i & (1 << j) != 0 {
        //             sum[i] = sum[i ^ (1 << j)] + employees[j as usize];
        //             break;
        //         }
        //     }
        // }
        // let mut dp = vec![0; size];
        // for i in 1..(size as i32) {
        //     if sum[i as usize] != 0 {
        //         continue;
        //     }
        //     dp[i as usize] = 1;
        //     let mut si = (i - 1) & i;
        //     while si != 0 {
        //         if dp[si as usize] != 0 {
        //             dp[i as usize] = i32::max(dp[i as usize], dp[si as usize] + 1);
        //         }
        //         si = (si - 1) & i;
        //     }
        // }
        // (len as i32) - dp[size - 1]
        let mut cnt = std::collections::HashMap::new();
        for t in &transactions {
            *cnt.entry(t[0]).or_insert(0) += t[2];
            *cnt.entry(t[1]).or_insert(0) -= t[2];
        }
        let mut acc = cnt.values().filter(|&x| *x != 0).cloned().collect();
        let mut ans = i32::MAX;
        pub fn helper(acc: &mut Vec<i32>, mut start: usize, cnt: i32, ans: &mut i32) {
            let n = acc.len();
            while start < n && acc[start] == 0 {
                start += 1;
            }
            if start == n {
                *ans = cnt.min(*ans);
                return;
            }
            for i in start + 1..n {
                if acc[i] * acc[start] < 0 {
                    acc[i] += acc[start];
                    helper(acc, start + 1, cnt + 1, ans);
                    acc[i] -= acc[start];
                }
            }
        }
        helper(&mut acc, 0, 0, &mut ans);
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_transfers_1() {
        assert_eq!(
            Solution::min_transfers(vec![vec![0, 1, 10], vec![2, 0, 5]]),
            2
        );
    }

    #[test]
    pub fn test_min_transfers_2() {
        assert_eq!(
            Solution::min_transfers(vec![
                vec![0, 1, 10],
                vec![1, 0, 1],
                vec![1, 2, 5],
                vec![2, 0, 5]
            ]),
            1
        );
    }
}
