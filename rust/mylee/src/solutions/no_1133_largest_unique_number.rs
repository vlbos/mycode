// 1133\. Largest Unique Number
// ============================

// Given an array of integers `A`, return the largest integer that only occurs once.

// If no integer occurs once, return -1.

// **Example 1:**

// **Input:** \[5,7,3,9,4,9,8,3,1\]
// **Output:** 8
// **Explanation:**
// The maximum integer in the array is 9 but it is repeated. The number 8 occurs only once, so it's the answer.

// **Example 2:**

// **Input:** \[9,9,8,8\]
// **Output:** \-1
// **Explanation:**
// There is no number that occurs only once.

// **Note:**

// 1.  `1 <= A.length <= 2000`
// 2.  `0 <= A[i] <= 1000`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   largest_unique_number(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort();
        let mut pre = a[a.len() - 1];
        let mut cnt = 0;
        for &v in a.iter().rev() {
            if pre == v {
                cnt += 1;
            } else {
                if cnt == 1 {
                    return pre;
                }
                cnt = 1;
                pre = v;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_largest_unique_number_1() {
        assert_eq!(
            8,
            Solution::largest_unique_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 1])
        );
    }
    #[test]
   pub fn  test_largest_unique_number_2() {
        assert_eq!(-1, Solution::largest_unique_number(vec![9, 9, 8, 8]));
    }
}
