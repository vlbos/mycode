// # [2907. Maximum Profitable Triplets With Increasing Prices I](https://leetcode.com/problems/maximum-profitable-triplets-with-increasing-prices-i)

// ## Description

// Given the 0-indexed arrays prices and profits of length n.
// There are n items in an store where the ith item has a price of prices[i] and a profit of profits[i].

// We have to pick three items with the following condition:

//
// 	prices[i]  < prices[j]  < prices[k] where i  < j  < k.
//

// If we pick items with indices i, j and k satisfying the above condition,
// the profit would be profits[i] + profits[j] + profits[k].

// Return the maximum profit we can get,
// and -1 if it 's not possible to pick three items with the given condition.

//
// Example 1:

//
// Input: prices = [10,2,3,4], profits = [100,2,7,10]
// Output: 19
// Explanation: We can 't pick the item with index i=0 since there are no indices j and k such that the condition holds.
// So the only triplet we can pick,
// are the items with indices 1, 2 and 3 and it 's a valid pick since prices[1]  < prices[2]  < prices[3].
// The answer would be sum of their profits which is 2 + 7 + 10 = 19.

// Example 2:

//
// Input: prices = [1,2,3,4,5], profits = [1,5,3,4,6]
// Output: 15
// Explanation: We can select any triplet of items since for each triplet of indices i, j and k such that i  < j  < k,
// the condition holds.
// Therefore the maximum profit we can get would be the 3 most profitable items which are indices 1, 3 and 4.
// The answer would be sum of their profits which is 5 + 4 + 6 = 15.

// Example 3:

//
// Input: prices = [4,3,2,1], profits = [33,20,19,87]
// Output: -1
// Explanation: We can 't select any triplet of indices such that the condition holds, so we return -1.
//

//
// Constraints:

//
// 	3  <= prices.length == profits.length  <= 2000
// 	1  <= prices[i]  <= 106
// 	1  <= profits[i]  <= 106
//

// ```rust
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
//
//     }
// }
// ```

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
        let update = |mut x: i32, v: i32, c: &mut Vec<i32>| {
            let n = c.len() as i32;
            while x < n {
                if c[x as usize] < v {
                    c[x as usize] = v;
                }
                x += x & -x;
            }
        };
        let query = |mut x: i32, c: &Vec<i32>| {
            let mut mx = 0;
            while x > 0 {
                if mx < c[x as usize] {
                    mx = c[x as usize];
                }
                x -= x & -x;
            }
            mx
        };
        let n = prices.len();
        let (mut left, mut right) = (vec![0; n], vec![0; n]);
        let mut s = prices.clone();
        s.sort_unstable();
        s.dedup();
        let m = s.len();
        let (mut tree1, mut tree2) = (vec![0; m + 1], vec![0; m + 1]);
        for (i, &x) in prices.iter().enumerate() {
            let x = s.partition_point(|&v| v < x) as i32;
            left[i] = query(x, &tree1);
            update(x + 1, profits[i], &mut tree1);
        }
        for (i, &x) in prices.iter().enumerate().rev() {
            let x = (m - s.partition_point(|&v| v < x)) as i32;
            right[i] = query(x, &tree2);
            update(x + 1, profits[i], &mut tree2);
        }
        left.into_iter()
            .zip(profits)
            .zip(right)
            .filter(|&((l, _), r)| l > 0 && r > 0)
            .map(|((l, x), r)| l + x + r)
            .max()
            .unwrap_or(-1)
    }
    pub fn max_profit2(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
        let mut ans = -1;
        let n = profits.len();
        for (j, &x) in profits.iter().enumerate() {
            let (mut left, mut right) = (0, 0);
            for i in 0..j {
                if prices[i] < prices[j] && left < profits[i] {
                    left = profits[i];
                }
            }
            for k in j + 1..n {
                if prices[j] < prices[k] && right < profits[k] {
                    right = profits[k];
                }
            }
            if left > 0 && right > 0 {
                ans = ans.max(left + x + right);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_profit_1() {
        assert_eq!(
            19,
            Solution::max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10])
        );
    }
    #[test]
    pub fn test_max_profit_2() {
        assert_eq!(
            15,
            Solution::max_profit(vec![1, 2, 3, 4, 5], vec![1, 5, 3, 4, 6])
        );
    }
    #[test]
    pub fn test_max_profit_3() {
        assert_eq!(
            -1,
            Solution::max_profit(vec![4, 3, 2, 1], vec![33, 20, 19, 87])
        );
    }
}
