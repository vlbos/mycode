/*
A Stepping Number is an integer such that all of its adjacent digits have an absolute difference of exactly 1. For example, 321 is a Stepping Number while 421 is not.

Given two integers low and high, find and return a sorted list of all the Stepping Numbers in the range [low, high] inclusive.


Example 1:
Input: low = 0, high = 21
Output: [0,1,2,3,4,5,6,7,8,9,10,12,21]


Constraints:
    0 <= low <= high <= 2 * 10^9


*/
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn count_stepping_numbers(low: i32, high: i32) -> Vec<i32> {
        Vec::new()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_stepping_numbers_1() {
        assert_eq!(Vec::<i32>::new(), Solution::count_stepping_numbers(0, 0));
    }
}
