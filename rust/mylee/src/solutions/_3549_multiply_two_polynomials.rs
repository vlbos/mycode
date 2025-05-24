
// ## [3549\. Multiply Two Polynomials 🔒](https://leetcode.com/problems/multiply-two-polynomials)

// ## Description

// You are given two integer arrays `poly1` and `poly2`, where the element at index `i` in each array represents the coefficient of `xi` in a polynomial.

// Let `A(x)` and `B(x)` be the polynomials represented by `poly1` and `poly2`, respectively.

// Return an integer array `result` representing the coefficients of the product polynomial `R(x) = A(x) * B(x)`, where `result[i]` denotes the coefficient of `xi` in `R(x)`.

// **Example 1:**

// **Input:** poly1 = \[3,2,5\], poly2 = \[1,4\]

// **Output:** \[3,14,13,20\]

// **Explanation:**

// +   `A(x) = 3 + 2x + 5x2` and `B(x) = 1 + 4x`
// +   `R(x) = (3 + 2x + 5x2) * (1 + 4x)`
// +   `R(x) = 3 * 1 + (3 * 4 + 2 * 1)x + (2 * 4 + 5 * 1)x2 + (5 * 4)x3`
// +   `R(x) = 3 + 14x + 13x2 + 20x3`
// +   Thus, result = `[3, 14, 13, 20]`.

// **Example 2:**

// **Input:** poly1 = \[1,0,-2\], poly2 = \[-1\]

// **Output:** \[-1,0,2\]

// **Explanation:**

// +   `A(x) = 1 + 0x - 2x2` and `B(x) = -1`
// +   `R(x) = (1 + 0x - 2x2) * (-1)`
// +   `R(x) = -1 + 0x + 2x2`
// +   Thus, result = `[-1, 0, 2]`.

// **Example 3:**

// **Input:** poly1 = \[1,5,-3\], poly2 = \[-4,2,0\]

// **Output:** \[-4,-18,22,-6,0\]

// **Explanation:**

// +   `A(x) = 1 + 5x - 3x2` and `B(x) = -4 + 2x + 0x2`
// +   `R(x) = (1 + 5x - 3x2) * (-4 + 2x + 0x2)`
// +   `R(x) = 1 * -4 + (1 * 2 + 5 * -4)x + (5 * 2 + -3 * -4)x2 + (-3 * 2)x3 + 0x4`
// +   `R(x) = -4 -18x + 22x2 -6x3 + 0x4`
// +   Thus, result = `[-4, -18, 22, -6, 0]`.

// **Constraints:**

// +   `1 <= poly1.length, poly2.length <= 5 * 104`
// +   `-103 <= poly1[i], poly2[i] <= 103`
// +   `poly1` and `poly2` contain at least one non-zero coefficient.

// // vector<long long> multiply(vector<int>& poly1, vector<int>& poly2) {



#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn multiply(poly1: Vec<i32>, poly2: Vec<i32>) -> Vec<i64> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_multiply_1() {
        assert_eq!(
            vec![3,14,13,20],
            Solution::multiply(vec![3,2,5], vec![1,4])
        );
    }
    #[test]
    pub fn test_multiply_2() {
        assert_eq!(
            vec![-1,0,2],
            Solution::multiply(vec![1,0,-2], vec![-1])
        );
    }
    #[test]
    pub fn test_multiply_3() {
        assert_eq!(
            vec![-4,-18,22,-6,0],
            Solution::multiply(vec![1,5,-3], vec![-4,2,0])
        );
    }
}
