// [2093\. Minimum Cost to Reach City With Discounts (Medium)](https://leetcode.com/problems/minimum-cost-to-reach-city-with-discounts/)[](https://leetcode.ca/2021-12-16-2093-Minimum-Cost-to-Reach-City-With-Discounts/#2093-minimum-cost-to-reach-city-with-discounts-medium)
// =============================================================================================================================================================================================================================================================================

// A series of highways connect `n` cities numbered from `0` to `n - 1`. You are given a 2D integer array `highways` where `highways[i] = [city1i, city2i, tolli]` indicates that there is a highway that connects `city1i` and `city2i`, allowing a car to go from `city1i` to `city2i` **and vice versa** for a cost of `tolli`.

// You are also given an integer `discounts` which represents the number of discounts you have. You can use a discount to travel across the `ith` highway for a cost of `tolli / 2` (**integer** **division**). Each discount may only be used **once**, and you can only use at most **one** discount per highway.

// Return _the **minimum total cost** to go from city_ `0` _to city_ `n - 1`_, or_ `-1` _if it is not possible to go from city_ `0` _to city_ `n - 1`_._

// **Example 1:**
// ![](https://assets.leetcode.com/uploads/2021/11/29/image-20211129222429-1.png)

// **Input:** n = 5, highways = \[\[0,1,4\],\[2,1,3\],\[1,4,11\],\[3,2,3\],\[3,4,2\]\], discounts = 1
// **Output:** 9
// **Explanation:**
// Go from 0 to 1 for a cost of 4.
// Go from 1 to 4 and use a discount for a cost of 11 / 2 = 5.
// The minimum cost to go from 0 to 4 is 4 + 5 = 9.

// **Example 2:**
// ![](https://assets.leetcode.com/uploads/2021/11/29/image-20211129222650-4.png)

// **Input:** n = 4, highways = \[\[1,3,17\],\[1,2,7\],\[3,2,5\],\[0,1,6\],\[3,0,20\]\], discounts = 20
// **Output:** 8
// **Explanation:**
// Go from 0 to 1 and use a discount for a cost of 6 / 2 = 3.
// Go from 1 to 2 and use a discount for a cost of 7 / 2 = 3.
// Go from 2 to 3 and use a discount for a cost of 5 / 2 = 2.
// The minimum cost to go from 0 to 3 is 3 + 3 + 2 = 8.

// **Example 3:**
// ![](https://assets.leetcode.com/uploads/2021/11/29/image-20211129222531-3.png)

// **Input:** n = 4, highways = \[\[0,1,3\],\[2,3,2\]\], discounts = 0
// **Output:** -1
// **Explanation:**
// It is impossible to go from 0 to 3 so return -1.

// **Constraints:**

// *   `2 <= n <= 1000`
// *   `1 <= highways.length <= 1000`
// *   `highways[i].length == 3`
// *   `0 <= city1i, city2i <= n - 1`
// *   `city1i != city2i`
// *   `0 <= tolli <= 105`
// *   `0 <= discounts <= 500`
// *   There are no duplicate highways.

// **Companies**:
// [Flipkart](https://leetcode.com/company/flipkart)

// **Related Topics**:
// [Graph](https://leetcode.com/tag/graph/), [Shortest Path](https://leetcode.com/tag/shortest-path/)

// **Similar Questions**:

// *   [Cheapest Flights Within K Stops (Medium)](https://leetcode.com/problems/cheapest-flights-within-k-stops/)
// *   [Connecting Cities With Minimum Cost (Medium)](https://leetcode.com/problems/connecting-cities-with-minimum-cost/)
// *   [Minimum Cost to Reach Destination in Time (Hard)](https://leetcode.com/problems/minimum-cost-to-reach-destination-in-time/)

// Solution 1. Dijkstra[](https://leetcode.ca/2021-12-16-2093-Minimum-Cost-to-Reach-City-With-Discounts/#solution-1-dijkstra)
// --------------------------------------------------------------------------------------------------------------------------

// `dist[i][j]` is the minimum cost going from node `0` to node `j` with `i` discounts. The answer is `dist[d][N - 1]`.

// We can use Dijkstra algorithm layer by layer from `0` discounts to `d` discounts.

// When we extending from node `u` to node `v`, we have two options:

// *   Don’t the discount on the edge `u -> v`. The new cost is `dist[i][u] + weight[u][v]`
// *   Use the discount on the edge `u -> v`. The new cost is `dist[i-1][u] + weight[u][v] / 2`.

// We pick the smaller one out of them, say `newCost`, and update `dist[i][v]` if `newCost < dist[i][v]`.

//     // OJ: https://leetcode.com/problems/minimum-cost-to-reach-city-with-discounts/
//     // Time: O(DElogE)
//     // Space: O(DN + E)
//     class Solution {
//     public:
//         int minimumCost(int n, vector<vector<int>>& E, int d) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
