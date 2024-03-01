// # [2473. Minimum Cost to Buy Apples](https://leetcode.com/problems/minimum-cost-to-buy-apples)
// ## Description

//  You are given a positive integer  n  representing  n  cities numbered from  1  to  n .
// You are also given a  2D  array  roads , where  roads[i] = [a i , b i , cost i ]  indicates that
// there is a  bidirectional  road between cities  a i   and  b i   with a cost of traveling equal to  cost i  .

//  You can buy apples in  any  city you want, but some cities have different costs to buy apples.
//  You are given the array  appleCost  where  appleCost[i]  is the cost of buying one apple from city  i .

//  You start at some city, traverse through various roads, and eventually buy  exactly  one apple from  any  city.
//  After you buy that apple, you have to return back to the city you  started  at,
// but now the cost of all the roads will be  multiplied  by a given factor  k .

//  Given the integer  k , return  an array   answer   of size   n
// where   answer[i]   is the  minimum  total cost to buy an apple if you start at city   i .

//  Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2400-2499/2473.Minimum%20Cost%20to%20Buy%20Apples/images/graph55.png" style="width: 241px; height: 309px;" />

//  Input:  n = 4, roads = [[1,2,4],[2,3,2],[2,4,5],[3,4,1],[1,3,4]], appleCost = [56,42,102,301], k = 2
//  Output:  [54,42,48,51]
//  Explanation:  The minimum cost for each starting city is the following:
// - Starting at city 1: You take the path 1 -> 2, buy an apple at city 2, and finally take the path 2 -> 1. The total cost is 4 + 42 + 4 * 2 = 54.
// - Starting at city 2: You directly buy an apple at city 2. The total cost is 42.
// - Starting at city 3: You take the path 3 -> 2, buy an apple at city 2, and finally take the path 2 -> 3. The total cost is 2 + 42 + 2 * 2 = 48.
// - Starting at city 4: You take the path 4 -> 3 -> 2 then you buy at city 2, and finally take the path 2 -> 3 -> 4. The total cost is 1 + 2 + 42 + 1 * 2 + 2 * 2 = 51.

//  Example 2:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2400-2499/2473.Minimum%20Cost%20to%20Buy%20Apples/images/graph4.png" style="width: 167px; height: 309px;" />

//  Input:  n = 3, roads = [[1,2,5],[2,3,1],[3,1,2]], appleCost = [2,3,1], k = 3
//  Output:  [2,3,1]
//  Explanation:  It is always optimal to buy the apple in the starting city.

//   Constraints:

// 	  2 <= n <= 1000
// 	  1 <= roads.length <= 1000
// 	  1 <= a i , b i  <= n
// 	  a i  != b i
// 	  1 <= cost i  <= 10^5
// 	  appleCost.length == n
// 	  1 <= appleCost[i] <= 10^5
// 	  1 <= k <= 100
// 	 There are no repeated edges.
//  vector<long long> min_cost(int n, vector<vector<int>>& roads, vector<int>& apple_cost, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, roads: Vec<Vec<i32>>, apple_cost: Vec<i32>, k: i32) -> Vec<i64> {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};
        let mut g = HashMap::<i32, Vec<(i32, i32)>>::new();
        for r in &roads {
            g.entry(r[0]).or_insert(Vec::new()).push((r[1], r[2]));
            g.entry(r[1]).or_insert(Vec::new()).push((r[0], r[2]));
        }
        let dijkstra = |i: i32| {
            let mut q = BinaryHeap::from([Reverse((0, i))]);
            let mut dist = HashMap::from([(i, 0)]);
            let mut ans = i64::MAX;
            while let Some(Reverse((d, u))) = q.pop() {
                ans = ans.min((apple_cost[u as usize - 1] as i64) + (d as i64) * (k as i64 + 1));
                for &(v, w) in g.get(&u).unwrap_or(&Vec::new()) {
                    if *dist.get(&v).unwrap_or(&i32::MAX) > dist[&u] + w {
                        dist.insert(v, dist[&u] + w);
                        q.push(Reverse((dist[&v], v)));
                    }
                }
            }
            ans
        };

        (1..=n).map(|i| dijkstra(i)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_cost_1() {
        assert_eq!(
            vec![54, 42, 48, 51],
            Solution::min_cost(
                4,
                vec![
                    vec![1, 2, 4],
                    vec![2, 3, 2],
                    vec![2, 4, 5],
                    vec![3, 4, 1],
                    vec![1, 3, 4]
                ],
                vec![56, 42, 102, 301],
                2
            )
        );
    }
    #[test]
    pub fn test_min_cost_2() {
        assert_eq!(
            vec![2, 3, 1],
            Solution::min_cost(
                3,
                vec![vec![1, 2, 5], vec![2, 3, 1], vec![3, 1, 2]],
                vec![2, 3, 1],
                3
            )
        );
    }
}
