/*
Given an array of integers A, return the largest integer that only occurs once.

If no integer occurs once, return -1.



Example 1:
Input: [5,7,3,9,4,9,8,3,1]
Output: 8
Explanation:
The maximum integer in the array is 9 but it is repeated. The number 8 occurs only once, so it's the answer.

Example 2:
Input: [9,9,8,8]
Output: -1
Explanation:
There is no number that occurs only once.



Note:
    1 <= A.length <= 2000
    0 <= A[i] <= 1000


*/ 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    pub fn largest_unique_number(a: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_unique_number_1() {
        assert_eq!(0, Solution::largest_unique_number(Vec::new()));
    }
}
