// # [3141. Maximum Hamming Distances 🔒](https://leetcode.com/problems/maximum-hamming-distances)

// ## Description

//

// Given an array nums and an integer m,
// with each element nums[i] satisfying 0 <= nums[i] < 2m, return an array answer.
// The answer array should be of the same length as nums,
//  where each element answer[i] represents the maximum Hamming distance between nums[i] and any other element nums[j] in the array.

// The Hamming distance between two binary integers is defined as the number of positions at which the corresponding bits differ (add leading zeroes if needed).

//
// Example 1:

//
// Input: nums = [9,12,9,11], m = 4

// Output: [2,3,2,3]

// Explanation:

// The binary representation of nums = [1001,1100,1001,1011].

// The maximum hamming distances for each index are:

//
// 	nums[0]: 1001 and 1100 have a distance of 2.
// 	nums[1]: 1100 and 1011 have a distance of 3.
// 	nums[2]: 1001 and 1100 have a distance of 2.
// 	nums[3]: 1011 and 1100 have a distance of 3.
//
//

// Example 2:

//
// Input: nums = [3,4,6,10], m = 4

// Output: [3,3,2,3]

// Explanation:

// The binary representation of nums = [0011,0100,0110,1010].

// The maximum hamming distances for each index are:

//
// 	nums[0]: 0011 and 0100 have a distance of 3.
// 	nums[1]: 0100 and 0011 have a distance of 3.
// 	nums[2]: 0110 and 1010 have a distance of 2.
// 	nums[3]: 1010 and 0100 have a distance of 3.
//
//

//
// Constraints:

//
// 	1 <= m <= 17
// 	2 <= nums.length <= 2m
// 	0 <= nums[i] < 2m
//

//     vector<int> max_hamming_distances(vector<int>& nums, int m) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_hamming_distances(nums: Vec<i32>, m: i32) -> Vec<i32> {
        let mut dist = vec![-1; 1 << m as usize];
        for &x in &nums {
            dist[x as usize] = 0;
        }
        let mut q: std::collections::VecDeque<_> = nums.iter().cloned().collect();
        let mut k = 1;
        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let x = q.pop_front().unwrap();
                for i in 0..m {
                    let y = x ^ (1 << i as usize);
                    if dist[y as usize] == -1 {
                        q.push_back(y);
                        dist[y as usize] = k;
                    }
                }
            }
            k += 1;
        }
        nums.into_iter()
            .map(|x| m - dist[(x ^ ((1 << m as usize) - 1)) as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_hamming_distances_1() {
        assert_eq!(
            vec![2, 3, 2, 3],
            Solution::max_hamming_distances(vec![9, 12, 9, 11], 4)
        );
    }
    #[test]
    pub fn test_max_hamming_distances_2() {
        assert_eq!(
            vec![3, 3, 2, 3],
            Solution::max_hamming_distances(vec![3, 4, 6, 10], 4)
        );
    }
}
