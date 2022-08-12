// 265\. Paint House II
// ====================

// There are a row of _n_ houses, each house can be painted with one of the _k_ colors.
// The cost of painting each house with a certain color is different.
// You have to paint all the houses such that no two adjacent houses have the same color.

// The cost of painting each house with a certain color is represented by a `_n_ x _k_` cost matrix.
// For example, `costs[0][0]` is the cost of painting house 0 with color 0;
//  `costs[1][2]` is the cost of painting house 1 with color 2, and so on...
// Find the minimum cost to paint all houses.

// **Note:**
// All costs are positive integers.

// **Example:**

// **Input:** \[\[1,5,3\],\[2,9,4\]\]
// **Output:** 5
// **Explanation:** Paint house 0 into color 0, paint house 1 into color 2. Minimum cost: 1 + 4 = 5;
//              Or paint house 0 into color 2, paint house 1 into color 0. Minimum cost: 3 + 2 = 5.

// **Follow up:**
// Could you solve it in _O_(_nk_) runtime?

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start

// pub  struct  CostRecord {
//     min_cost: i32,
//     second_min_cost: i32,
//     min_id: usize,
// }

// impl CostRecord {
//     pub fn   new(init_cost: i32) -> Self {
//         Self {
//             min_cost: init_cost,
//             second_min_cost: init_cost,
//             min_id: usize::max_value(),
//         }
//     }

//     pub fn   add(&mut self, last_record: &CostRecord, id: usize, cost: i32) {
//         let new_all_cost = (if id == last_record.min_id {
//             last_record.second_min_cost
//         } else {
//             last_record.min_cost
//         }) + cost;
//         if new_all_cost < self.min_cost {
//             self.second_min_cost = self.min_cost;
//             self.min_cost = new_all_cost;
//             self.min_id = id;
//         } else if new_all_cost < self.second_min_cost {
//             self.second_min_cost = new_all_cost;
//         }
//     }
// }

impl Solution {
    pub fn   min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        // if costs.len() == 0 {
        //     return 0;
        // }
        // let mut record = CostRecord::new(0);
        // for i in 0..costs.len() {
        //     let mut new_record = CostRecord::new(i32::max_value());
        //     for j in 0..costs[i].len() {
        //         new_record.add(&record, j, costs[i][j]);
        //     }
        //     record = new_record;
        // }
        // record.min_cost
        let mut dp = costs[0].clone();
        for cost in &costs[1..] {
            let mut new_dp = vec![0; dp.len()];
            for i in 0..cost.len() {
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

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_min_cost_ii() {
        let costs = vec![vec![1, 5, 3], vec![2, 9, 4]];
        assert_eq!(Solution::min_cost_ii(costs), 5);
    }
}
