// 3893. Maximum Team Size with Overlapping Intervals
// ## Description
// You are given two integer arrays `startTime` and `endTime` of length `n`.
// * `startTime[i]` represents the start time of the `ith` employee.
// * `endTime[i]` represents the end time of the `ith` employee.
// Two employees `i` and `j` can interact if their time intervals **overlap**. Two intervals are considered overlapping if they share **at least one** common time point.
// A team is **valid** if there exists **at least one** employee in the team who can interact with every other member of the team.
// Return an integer denoting the **maximum** possible size of such a team.

// **Example 1:**
// **Input:** startTime = [1,2,3], endTime = [4,5,6]
// **Output:** 3
// **Explanation:**
// * For `i = 0` with interval `[1, 4]`.
// * It overlaps with `i = 1` having interval `[2, 5]` and `i = 2` having interval `[3, 6]`.
// * Thus, index 0 can interact with all other indices, so the team size is 3.
// **Example 2:**
// **Input:** startTime = [2,5,8], endTime = [3,7,9]
// **Output:** 1
// **Explanation:**
// * For `i = 0`, interval `[2, 3]` does not overlap with `[5, 7]` or `[8, 9]`.
// * For `i = 1`, interval `[5, 7]` does not overlap with `[2, 3]` or `[8, 9]`.
// * For `i = 2`, interval `[8, 9]` does not overlap with `[2, 3]` or `[5, 7]`.
// * Thus, no index can interact with others, so the maximum team size is 1.
// **Example 3:**
// **Input:** startTime = [3,4,6], endTime = [8,5,7]
// **Output:** 3
// **Explanation:**
// * For `i = 0` with interval `[3, 8]`.
// * It overlaps with `i = 1` having interval `[4, 5]` and `i = 2` having interval `[6, 7]`.
// * Thus, index 0 can interact with all other indices, so the team size is 3.

// **Constraints:**
// * `1 \<= n == startTime.length == endTime.length \<= 105`
// * `0 \<= startTime[i] \<= endTime[i] \<= 109`

// int maximum_team_size(vector<int>& start_time, vector<int>& endTime) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn maximum_team_size(start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_team_size_1() {
        assert_eq!(
            3,
            Solution::maximum_team_size(vec![1,2,3], vec![4,5,6])
        );
    }
    #[test]
    pub fn test_maximum_team_size_2() {
        assert_eq!(
            1,
            Solution::maximum_team_size(vec![2,5,8], vec![3,7,9])
        );
    }
    #[test]
    pub fn test_maximum_team_size_3() {
        assert_eq!(
            3,
            Solution::maximum_team_size(vec![3,4,6], vec![8,5,7])
        );
    }

}
