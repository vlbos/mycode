// 484\. Find Permutation
// ======================

// By now, you are given a **secret signature** consisting of character 'D' and 'I'. 'D' represents a decreasing relationship between two numbers,
// 'I' represents an increasing relationship between two numbers.
// And our **secret signature** was constructed by a special integer array,
// which contains uniquely all the different number from 1 to n (n is the length of the secret signature plus 1).
// For example, the secret signature "DI" can be constructed by array \[2,1,3\] or \[3,1,2\],
// but won't be constructed by array \[3,2,4\] or \[2,1,3,4\],
// which are both illegal constructing special string that can't represent the "DI" **secret signature**.

// On the other hand, now your job is to find the lexicographically smallest permutation of \[1, 2, ... n\] could refer to the given **secret signature** in the input.

// **Example 1:**

// **Input:** "I"
// **Output:** \[1,2\]
// **Explanation:** \[1,2\] is the only legal initial spectial string can construct secret signature "I", where the number 1 and 2 construct an increasing relationship.

// **Example 2:**

// **Input:** "DI"
// **Output:** \[2,1,3\]
// **Explanation:** Both \[2,1,3\] and \[3,1,2\] can construct the secret signature "DI",
// but since we want to find the one with the smallest lexicographical permutation, you need to output \[2,1,3\]

// **Note:**

// *   The input string will only contain the character 'D' and 'I'.
// *   The length of input string is a positive integer and will not exceed 10,000

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        // let chars = s.chars().collect::<Vec<char>>();
        // let len = chars.len();
        // let mut res = (1..=(len + 1) as i32).into_iter().collect::<Vec<i32>>();
        // let mut i = 1usize;
        // while i <= len {
        //     let j = i;
        //     while i <= len && chars[i - 1] == 'D' {
        //         i += 1;
        //     }
        //     Solution::reverse(&mut res[(j - 1)..i]);
        //     i += 1;
        // }
        // res
        let n = s.len() + 1;
        let mut ans: Vec<i32> = (1..=n as i32).collect();
        let mut i = 0;
        let bs = s.as_bytes();
        while i < n {
            let j = i;
            while i < n - 1 && bs[i] == b'D' {
                i += 1;
            }
            ans[j..=i].reverse();
            i += 1;
        }
        ans
    }

    // fn reverse(arr: &mut [i32]) {
    //     let len = arr.len();
    //     let mid = len / 2;
    //     for i in 0..mid {
    //         arr.swap(i, len - 1 - i);
    //     }
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_permutation_1() {
        assert_eq!(Solution::find_permutation(String::from("I")), vec![1, 2]);
    }

    #[test]
    fn test_find_permutation_2() {
        assert_eq!(
            Solution::find_permutation(String::from("DI")),
            vec![2, 1, 3]
        );
    }
}
