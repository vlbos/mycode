// # [2113. Elements in Array After Removing and Replacing Elements](https://leetcode.com/problems/elements-in-array-after-removing-and-replacing-elements)

// ## Description

// You are given a 0-indexed integer array nums. Initially on minute 0, the array is unchanged. Every minute,
// the leftmost element in nums is removed until no elements remain.
// Then, every minute, one element is appended to the end of nums, in the order they were removed in, until the original array is restored.
// This process repeats indefinitely.

//
// 	For example, the array [0,1,2] would change as follows: [0,1,2]-> [1,2]-> [2]-> []-> [0]-> [0,1]-> [0,1,2]-> [1,2]-> [2]-> []-> [0]-> [0,1]-> [0,1,2]-> ...
//

// You are also given a 2D integer array queries of size n where queries[j] = [timej, indexj]. The answer to the jth query is:

//
// 	nums[indexj] if indexj < nums.length at minute timej
// 	-1 if indexj >= nums.length at minute timej
//

// Return an integer array ans of size n where ans[j] is the answer to the jth query.

// Example 1:

//
// Input: nums = [0,1,2], queries = [[0,2],[2,0],[3,2],[5,0]]
// Output: [2,2,-1,0]
// Explanation:
// Minute 0: [0,1,2] - All elements are in the nums.
// Minute 1: [1,2]   - The leftmost element, 0, is removed.
// Minute 2: [2]     - The leftmost element, 1, is removed.
// Minute 3: []      - The leftmost element, 2, is removed.
// Minute 4: [0]     - 0 is added to the end of nums.
// Minute 5: [0,1]   - 1 is added to the end of nums.

// At minute 0, nums[2] is 2.
// At minute 2, nums[0] is 2.
// At minute 3, nums[2] does not exist.
// At minute 5, nums[0] is 0.
//

// Example 2:

//
// Input: nums = [2], queries = [[0,0],[1,0],[2,0],[3,0]]
// Output: [2,-1,2,-1]
// Minute 0: [2] - All elements are in the nums.
// Minute 1: []  - The leftmost element, 2, is removed.
// Minute 2: [2] - 2 is added to the end of nums.
// Minute 3: []  - The leftmost element, 2, is removed.

// At minute 0, nums[0] is 2.
// At minute 1, nums[0] does not exist.
// At minute 2, nums[0] is 2.
// At minute 3, nums[0] does not exist.
//

// Constraints:

//
// 	1 <= nums.length <= 100
// 	0 <= nums[i] <= 100
// 	n == queries.length
// 	1 <= n <= 105
// 	queries[j].length == 2
// 	0 <= timej <= 105
// 	0 <= indexj < nums.length
//
//  vector<int> element_in_nums(vector<int>& nums, vector<vector<int>>& queries) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn element_in_nums(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let n2 = 2 * n;
        let mut ans = Vec::new();
        for q in &queries {
            let (m, index) = (q[0] as usize, q[1] as usize);
            let i = m % n2;
            let len = if i < n {
                n - i
            } else if i > n {
                i - n
            } else {
                0
            };
            ans.push(if index < len {
                let j = index + if i < n { i } else { 0 };
                nums[j]
            } else {
                -1
            });
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_element_in_nums_1() {
        assert_eq!(
            vec![2, 2, -1, 0],
            Solution::element_in_nums(
                vec![0, 1, 2],
                vec![vec![0, 2], vec![2, 0], vec![3, 2], vec![5, 0]]
            )
        );
    }
    #[test]
    pub fn test_element_in_nums_2() {
        assert_eq!(
            vec![2, -1, 2, -1],
            Solution::element_in_nums(
                vec![2],
                vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0]]
            )
        );
    }
}
