// 256\. Paint House
// =================

// There are a row of _n_ houses, each house can be painted with one of the three colors: red, blue or green.
//  The cost of painting each house with a certain color is different.
// You have to paint all the houses such that no two adjacent houses have the same color.

// The cost of painting each house with a certain color is represented by a `_n_ x _3_` cost matrix.
// For example, `costs[0][0]` is the cost of painting house 0 with color red; `costs[1][2]` is the cost of painting house 1 with color green,
// and so on... Find the minimum cost to paint all houses.

// **Note:**
// All costs are positive integers.

// **Example:**

// **Input:** \[\[17,2,17\],\[16,16,5\],\[14,3,19\]\]
// **Output:** 10
// **Explanation:** Paint house 0 into blue, paint house 1 into green, paint house 2 into blue.
//              Minimum cost: 2 + 5 + 3 = 10.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Apple](https://leetcode.ca/tags/#Apple) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Twitter](https://leetcode.ca/tags/#Twitter)
struct Solution;
// @lc code=start

const COLOR_SIZE: usize = 3;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        // let mut dp: [[i32; COLOR_SIZE]; 2] = [[0, 0, 0], [0, 0, 0]];
        // let mut iter = 0;
        // for c in costs {
        //     let another_iter = iter;
        //     iter = (iter + 1) % 2;
        //     for i in 0..COLOR_SIZE {
        //         dp[iter][i] = i32::min(
        //             dp[another_iter][(i + 1) % COLOR_SIZE],
        //             dp[another_iter][(i + 2) % COLOR_SIZE],
        //         ) + c[i];
        //     }
        // }
        // dp[iter]
        //     .iter()
        //     .fold(i32::max_value(), |sum, curr| i32::min(sum, *curr))
        let mut dp = costs[0].clone();
        for cost in &costs[1..] {
            let mut new_dp = vec![0; 3];
            for i in 0..3 {
                new_dp[i] = cost[i]
                    + *dp
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .min_by_key(|(_, &v)| v)
                        .unwrap()
                        .1;
            }
            dp = new_dp;
        }
        dp.into_iter().min().unwrap()
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost() {
        let inputs = vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]];
        assert_eq!(Solution::min_cost(inputs), 10);
    }
}
