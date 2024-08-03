// # [3135. Equalize Strings by Adding or Removing Characters at Ends 🔒](https://leetcode.com/problems/equalize-strings-by-adding-or-removing-characters-at-ends)

// ## Description

// Given two strings initial and target,
// your task is to modify initial by performing a series of operations to make it equal to target.

// In one operation, you can add or remove one character only at the beginning or the end of the string initial.

// Return the minimum number of operations required to transform initial into target.

//
// Example 1:

// Input: initial = "abcde", target = "cdef"

// Output: 3

// Explanation:

// Remove 'a' and 'b' from the beginning of initial, then add 'f' to the end.

// Example 2:

// Input: initial = "axxy", target = "yabx"

// Output: 6

// Explanation:

// 			Operation
// 			Resulting String

// 			Add 'y' to the beginning
// 			"yaxxy"

// 			Remove from end
// 			"yaxx"

// 			Remove from end
// 			"yax"

// 			Remove from end
// 			"ya"

// 			Add 'b' to the end
// 			"yab"

// 			Add 'x' to the end
// 			"yabx"

// Example 3:

// Input: initial = "xyz", target = "xyz"

// Output: 0

// Explanation:

// No operations are needed as the strings are already equal.

//
// Constraints:

// 	1 <= initial.length, target.length <= 1000
// 	initial and target consist only of lowercase English letters.

//     int min_operations(string initial, string target) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_operations(initial: String, target: String) -> i32 {
        let (m, n) = (initial.len(), target.len());
        let mut f = vec![vec![0; n + 1]; m + 1];
        let mut mx = 0;
        for (i, a) in initial.chars().enumerate() {
            for (j, b) in target.chars().enumerate() {
                if a == b {
                    f[i + 1][j + 1] = f[i][j] + 1;
                    mx = mx.max(f[i + 1][j + 1]);
                }
            }
        }
        (m + n) as i32 - mx * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(
            3,
            Solution::min_operations(String::from("abcde"), String::from("cdef"))
        );
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(
            6,
            Solution::min_operations(String::from("axxy"), String::from("yabx"))
        );
    }
    #[test]
    pub fn test_min_operations_3() {
        assert_eq!(
            0,
            Solution::min_operations(String::from("xyz"), String::from("xyz"))
        );
    }
}
