// # [2247. Maximum Cost of Trip With K Highways](https://leetcode.com/problems/maximum-cost-of-trip-with-k-highways)

// ## Description

// A series of highways connect n cities numbered from 0 to n - 1.
//  You are given a 2D integer array highways where highways[i] = [city1i, city2i, tolli] indicates that there is a highway that connects city1i and city2i,
// allowing a car to go from city1i to city2i and vice versa for a cost of tolli.

// You are also given an integer k. You are going on a trip that crosses exactly k highways.
//  You may start at any city, but you may only visit each city at most once during your trip.

// Return the maximum cost of your trip. If there is no trip that meets the requirements, return -1.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2247.Maximum%20Cost%20of%20Trip%20With%20K%20Highways/images/image-20220418173304-1.png" style="height: 200px; width: 327px;" />
//
// Input: n = 5, highways = [[0,1,4],[2,1,3],[1,4,11],[3,2,3],[3,4,2]], k = 3
// Output: 17
// Explanation:
// One possible trip is to go from 0 -> 1 -> 4 -> 3. The cost of this trip is 4 + 11 + 2 = 17.
// Another possible trip is to go from 4 -> 1 -> 2 -> 3. The cost of this trip is 11 + 3 + 3 = 17.
// It can be proven that 17 is the maximum possible cost of any valid trip.

// Note that the trip 4 -> 1 -> 0 -> 1 is not allowed because you visit the city 1 twice.

//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2247.Maximum%20Cost%20of%20Trip%20With%20K%20Highways/images/image-20220418173342-2.png" style="height: 200px; width: 217px;" />
//
// Input: n = 4, highways = [[0,1,3],[2,3,2]], k = 2
// Output: -1
// Explanation: There are no valid trips of length 2, so return -1.
//

// Constraints:

//
// 	2 <= n <= 15
// 	1 <= highways.length <= 50
// 	highways[i].length == 3
// 	0 <= city1i, city2i <= n - 1
// 	city1i != city2i
// 	0 <= tolli <= 100
// 	1 <= k <= 50
// 	There are no duplicate highways.
//

// int maximum_cost(int n, vector<vector<int>>& highways, int k) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximum_cost(n: i32, highways: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let n1 = 1 << n;
        let mut g = vec![Vec::new(); n];
        let mut dp = vec![-1; n1];
        let mut gct = vec![vec![0; n]; n1];
        for e in &highways {
            let (u, v, t) = (e[0] as usize, e[1] as usize, e[2]);
            g[u].push((v, t));
            g[v].push((u, t));
            let state = (1 << u) + (1 << v);
            dp[state] = t;
            gct[state][u] += 1;
            gct[state][v] += 1;
        }
        let mut ans = -1;
        for st1 in 0..n1 {
            for j in 0..format!("{:b}", st1).len() {
                if st1 & (1 << j) == 0 {
                    continue;
                }
                let st2 = st1 ^ (1 << j);
                for &(v, t) in &g[j] {
                    if st2 & (1 << v) == 0 || dp[st2] == -1 {
                        continue;
                    }
                    if gct[st2][v] + 1 > 2 {
                        continue;
                    }
                    if dp[st2] + t > dp[st1] {
                        gct[st1] = gct[st2].clone();
                        gct[st1][j] += 1;
                        gct[st1][v] += 1;
                        dp[st1] = dp[st2] + t;
                    }
                }
            }
            if st1.count_ones() == k as u32 + 1 {
                ans = ans.max(dp[st1]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_cost_1() {
        assert_eq!(
            17,
            Solution::maximum_cost(
                5,
                vec![
                    vec![0, 1, 4],
                    vec![2, 1, 3],
                    vec![1, 4, 11],
                    vec![3, 2, 3],
                    vec![3, 4, 2]
                ],
                3
            )
        );
    }
    #[test]
    pub fn test_maximum_cost_2() {
        assert_eq!(
            -1,
            Solution::maximum_cost(4, vec![vec![0, 1, 3], vec![2, 3, 2]], 2)
        );
    }
}
