// # [2323. Find Minimum Time to Finish All Jobs II](https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs-ii)

// ## Description

// You are given two 0-indexed integer arrays jobs and workers of equal length,
// where jobs[i] is the amount of time needed to complete the ith job, and workers[j] is the amount of time the jth worker can work each day.

// Each job should be assigned to exactly one worker, such that each worker completes exactly one job.

// Return the minimum number of days needed to complete all the jobs after assignment.

// Example 1:

//
// Input: jobs = [5,2,4], workers = [1,7,5]
// Output: 2
// Explanation:
// - Assign the 2nd worker to the 0th job. It takes them 1 day to finish the job.
// - Assign the 0th worker to the 1st job. It takes them 2 days to finish the job.
// - Assign the 1st worker to the 2nd job. It takes them 1 day to finish the job.
// It takes 2 days for all the jobs to be completed, so return 2.
// It can be proven that 2 days is the minimum number of days needed.
//

// Example 2:

//
// Input: jobs = [3,18,15,9], workers = [6,5,1,3]
// Output: 3
// Explanation:
// - Assign the 2nd worker to the 0th job. It takes them 3 days to finish the job.
// - Assign the 0th worker to the 1st job. It takes them 3 days to finish the job.
// - Assign the 1st worker to the 2nd job. It takes them 3 days to finish the job.
// - Assign the 3rd worker to the 3rd job. It takes them 3 days to finish the job.
// It takes 3 days for all the jobs to be completed, so return 3.
// It can be proven that 3 days is the minimum number of days needed.
//

// Constraints:

//
// 	n == jobs.length == workers.length
// 	1 <= n <= 10^5
// 	1 <= jobs[i], workers[i] <= 10^5
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def minimum_time(self, jobs: List[int], workers: List[int]) -> int:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimum_time(jobs: Vec<i32>, workers: Vec<i32>) -> i32 {
        let (mut jobs, mut workers) = (jobs, workers);
        jobs.sort();
        workers.sort();
        jobs.into_iter()
            .zip(workers)
            .map(|(j, w)| (j - 1) / w + 1)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_time_1() {
        assert_eq!(2, Solution::minimum_time(vec![5, 2, 4], vec![1, 7, 5]));
    }
    #[test]
    pub fn test_minimum_time_2() {
        assert_eq!(
            3,
            Solution::minimum_time(vec![3, 18, 15, 9], vec![6, 5, 1, 3])
        );
    }
}
