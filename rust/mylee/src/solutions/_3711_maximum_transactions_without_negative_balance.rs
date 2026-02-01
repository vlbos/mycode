// # 3711. Maximum Transactions Without Negative Balance 🔒

// Description
// -----------

// You are given an integer array `transactions`, where `transactions[i]` represents the amount of the `ith` transaction:

// *   A positive value means money is **received**.
// *   A negative value means money is **sent**.

// The account starts with a balance of 0, and the balance **must never become negative**.
// Transactions must be considered in the given order, but you are allowed to skip some transactions.

// Return an integer denoting the **maximum number of transactions** that can be performed without the balance ever going negative.

// **Example 1:**

// **Input:** transactions = \[2,-5,3,-1,-2\]

// **Output:** 4

// **Explanation:**

// One optimal sequence is `[2, 3, -1, -2]`, balance: `0 → 2 → 5 → 4 → 2`.

// **Example 2:**

// **Input:** transactions = \[-1,-2,-3\]

// **Output:** 0

// **Explanation:**

// All transactions are negative. Including any would make the balance negative.

// **Example 3:**

// **Input:** transactions = \[3,-2,3,-2,1,-1\]

// **Output:** 6

// **Explanation:**

// All transactions can be taken in order, balance: `0 → 3 → 1 → 4 → 2 → 3 → 2`.

// **Constraints:**

// *   `1 <= transactions.length <= 105`
// *   `-109 <= transactions[i] <= 109`

// // int max_transactions(vector<int>& transactions) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn max_transactions(transactions: Vec<i32>) -> i32 {
        use std::collections::BTreeMap; //, btree_map::OccupiedEntry
        let mut s = BTreeMap::new();
        let (mut ans, mut balance) = (transactions.len() as i32, 0);
        for t in transactions {
            balance += t;
            *s.entry(t).or_insert(0) += 1;
            while balance < 0 {
                if let Some(mut o) = s.first_entry() {
                    balance -= *o.key();
                    *o.get_mut() -= 1;
                    if *o.get() == 0 {
                        o.remove_entry();
                    }
                }
                ans -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_transactions_1() {
        assert_eq!(4, Solution::max_transactions(vec![2, -5, 3, -1, -2]));
    }
    #[test]
    pub fn test_max_transactions_2() {
        assert_eq!(0, Solution::max_transactions(vec![-1, -2, -3]));
    }
    #[test]
    pub fn test_max_transactions_3() {
        assert_eq!(6, Solution::max_transactions(vec![3, -2, 3, -2, 1, -1]));
    }
}
