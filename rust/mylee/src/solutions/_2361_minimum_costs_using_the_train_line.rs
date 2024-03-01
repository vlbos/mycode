// # [2361. Minimum Costs Using the Train Line](https://leetcode.com/problems/minimum-costs-using-the-train-line)

// ## Description

// A train line going through a city has two routes, the regular route and the express route.
// Both routes go through the same n + 1 stops labeled from 0 to n.
//  Initially, you start on the regular route at stop 0.

// You are given two 1-indexed integer arrays regular and express, both of length n.
// regular[i] describes the cost it takes to go from stop i - 1 to stop i using the regular route,
// and express[i] describes the cost it takes to go from stop i - 1 to stop i using the express route.

// You are also given an integer express Cost which represents the cost to transfer from the regular route to the express route.

// Note that:

//
// 	There is no cost to transfer from the express route back to the regular route.
// 	You pay expressCost every time you transfer from the regular route to the express route.
// 	There is no extra cost to stay on the express route.
//

// Return a 1-indexed array costs of length n, where costs[i] is the minimum cost to reach stop i from stop 0.

// Note that a stop can be counted as reached from either route.

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2361.Minimum%20Costs%20Using%20the%20Train%20Line/images/ex1drawio.png" style="width: 442px; height: 150px;" />
//
// Input: regular = [1,6,9,5], express = [5,2,3,10], expressCost = 8
// Output: [1,7,14,19]
// Explanation: The diagram above shows how to reach stop 4 from stop 0 with minimum cost.
// - Take the regular route from stop 0 to stop 1, costing 1.
// - Take the express route from stop 1 to stop 2, costing 8 + 2 = 10.
// - Take the express route from stop 2 to stop 3, costing 3.
// - Take the regular route from stop 3 to stop 4, costing 5.
// The total cost is 1 + 10 + 3 + 5 = 19.
// Note that a different route could be taken to reach the other stops with minimum cost.
//

// Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2361.Minimum%20Costs%20Using%20the%20Train%20Line/images/ex2drawio.png" style="width: 346px; height: 150px;" />
//
// Input: regular = [11,5,13], express = [7,10,6], expressCost = 3
// Output: [10,15,24]
// Explanation: The diagram above shows how to reach stop 3 from stop 0 with minimum cost.
// - Take the express route from stop 0 to stop 1, costing 3 + 7 = 10.
// - Take the regular route from stop 1 to stop 2, costing 5.
// - Take the express route from stop 2 to stop 3, costing 3 + 6 = 9.
// The total cost is 10 + 5 + 9 = 24.
// Note that the expressCost is paid again to transfer back to the express route.
//

// Constraints:

//
// 	n == regular.length == express.length
// 	1 <= n <= 105
// 	1 <= regular[i], express[i], expressCost <= 105
//

// vector<long long> minimum_costs(vector<int>& regular, vector<int>& express,
//                                  int express_cost) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn minimum_costs(regular: Vec<i32>, express: Vec<i32>, express_cost: i32) -> Vec<i64> {
        let mut ans = Vec::new();
        let mut dp = (0, express_cost as i64);
        for (r, e) in regular.into_iter().zip(express) {
            let (rdp, edp) = dp;
            dp = (
                rdp.min(edp) + r as i64,
                edp.min(rdp + express_cost as i64) + e as i64,
            );
            ans.push(dp.0.min(dp.1));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_costs_1() {
        assert_eq!(
            vec![1, 7, 14, 19],
            Solution::minimum_costs(vec![1, 6, 9, 5], vec![5, 2, 3, 10], 8)
        );
    }
    #[test]
    pub fn test_minimum_costs_2() {
        assert_eq!(
            vec![10, 15, 24],
            Solution::minimum_costs(vec![11, 5, 13], vec![7, 10, 6], 3)
        );
    }
}
