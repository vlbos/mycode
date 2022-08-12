// 1064\. Fixed Point
// ==================

// Given an array `A` of distinct integers sorted in ascending order, return the smallest index `i` that satisfies `A[i] == i`.  Return `-1` if no such `i` exists.

// **Example 1:**

// **Input:** \[-10,-5,0,3,7\]
// **Output:** 3
// **Explanation:**
// For the given array, `A[0] = -10, A[1] = -5, A[2] = 0, A[3] = 3`, thus the output is 3.

// **Example 2:**

// **Input:** \[0,2,5,8,17\]
// **Output:** 0
// **Explanation:**
// `A[0] = 0`, thus the output is 0.

// **Example 3:**

// **Input:** \[-10,-5,3,4,7,9\]
// **Output:** \-1
// **Explanation:**
// There is no such `i` that `A[i] = i`, thus the output is -1.

// **Note:**

// 1.  `1 <= A.length < 10^4`
// 2.  `-10^9 <= A[i] <= 10^9`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Uber](https://leetcode.ca/tags/#Uber)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   fixed_point(a: Vec<i32>) -> i32 {
        for (i, &v) in a.iter().enumerate() {
            if i as i32 == v {
                return v;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_fixed_point_1() {
        assert_eq!(3, Solution::fixed_point(vec![-10, -5, 0, 3, 7]));
    }
    #[test]
   pub fn  test_fixed_point_2() {
        assert_eq!(0, Solution::fixed_point(vec![0, 2, 5, 8, 17]));
    }
    #[test]
   pub fn  test_fixed_point_3() {
        assert_eq!(-1, Solution::fixed_point(vec![-10, -5, 3, 4, 7, 9]));
    }
}
