// 1118\. Number of Days in a Month
// ================================

// Given a year `Y` and a month `M`, return how many days there are in that month.

// **Example 1:**

// **Input:** Y = 1992, M = 7
// **Output:** 31

// **Example 2:**

// **Input:** Y = 2000, M = 2
// **Output:** 29

// **Example 3:**

// **Input:** Y = 1900, M = 2
// **Output:** 28

// **Note:**

// 1.  `1583 <= Y <= 2100`
// 2.  `1 <= M <= 12`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn number_of_days(year: i32, month: i32) -> i32 {
        if month == 2 {
            if year % 4 == 0 && (year % 100 > 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        } else if (month % 2 == 0) ^ (month < 8) {
            31
        } else {
            30
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_number_of_days_1() {
        assert_eq!(31, Solution::number_of_days(1992, 7));
    }

    #[test]
    pub fn test_number_of_days_2() {
        assert_eq!(29, Solution::number_of_days(2000, 2));
    }

    #[test]
    pub fn test_number_of_days_3() {
        assert_eq!(28, Solution::number_of_days(1900, 2));
    }
}
