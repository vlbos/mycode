// # [2297. Jump Game VIII](https://leetcode.com/problems/jump-game-viii)

// ## Description

// You are given a 0-indexed integer array nums of length n. 
// You are initially standing at index 0. 
// You can jump from index i to index j where i < j if:

//
// 	nums[i] <= nums[j] and nums[k] < nums[i] for all indexes k in the range i < k < j, or
// 	nums[i] > nums[j] and nums[k] >= nums[i] for all indexes k in the range i < k < j.
//

// You are also given an integer array costs of length n where costs[i] denotes the cost of jumping to index i.

// Return the minimum cost to jump to the index n - 1.

// Example 1:

//
// Input: nums = [3,2,4,4,1], costs = [3,7,6,4,2]
// Output: 8
// Explanation: You start at index 0.
// - Jump to index 2 with a cost of costs[2] = 6.
// - Jump to index 4 with a cost of costs[4] = 2.
// The total cost is 8. It can be proven that 8 is the minimum cost needed.
// Two other possible paths are from index 0 -> 1 -> 4 and index 0 -> 2 -> 3 -> 4.
// These have a total cost of 9 and 12, respectively.
//

// Example 2:

//
// Input: nums = [0,1,2], costs = [1,1,1]
// Output: 2
// Explanation: Start at index 0.
// - Jump to index 1 with a cost of costs[1] = 1.
// - Jump to index 2 with a cost of costs[2] = 1.
// The total cost is 2. Note that you cannot jump directly from index 0 to index 2 because nums[0] <= nums[1].
//

// Constraints:

//
// 	n == nums.length == costs.length
// 	1 <= n <= 10^5
// 	0 <= nums[i], costs[i] <= 10^5
//
//  long long min_cost(vector<int>& nums, vector<int>& costs) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_cost(nums: Vec<i32>,costs: Vec<i32>) -> i64 {
        let n=nums.len();
        let mut dp=vec![i64::MAX;n];
        let (mut xs,mut ms)=(Vec::new(),Vec::new());
        dp[0]=0;
        for (i,&num) in nums.iter().enumerate(){
            while !xs.is_empty() && num>=nums[*xs.last().unwrap()]{
                dp[i]=dp[i].min(dp[*xs.last().unwrap()]+costs[i] as i64);
            xs.pop();
            }
            while !ms.is_empty() && num<nums[*ms.last().unwrap()]{
                dp[i]=dp[i].min(dp[*ms.last().unwrap()]+costs[i]  as i64);
                ms.pop();
            }
            xs.push(i);
            ms.push(i);
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_cost_1() {
        assert_eq!(
           8,
            Solution::min_cost(
               vec![3,2,4,4,1],  vec![3,7,6,4,2]
            )
        );
    }
    #[test]
    pub fn test_min_cost_2() {
 assert_eq!(
            2,
            Solution::min_cost(
               vec![0,1,2], vec![1,1,1]
            )
        );
    }
    
}
