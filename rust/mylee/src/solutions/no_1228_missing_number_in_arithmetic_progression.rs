// 1228\. Missing Number In Arithmetic Progression
// ===============================================

// In some array `arr`, the values were in arithmetic progression: the values `arr[i+1] - arr[i]` are all equal for every `0 <= i < arr.length - 1`.

// Then, a value from `arr` was removed that **was not the first or last value in the array**.

// Return the removed value.

// **Example 1:**

// **Input:** arr = \[5,7,11,13\]
// **Output:** 9
// **Explanation:** The previous array was \[5,7,**9**,11,13\].

// **Example 2:**

// **Input:** arr = \[15,13,12\]
// **Output:** 14
// **Explanation:** The previous array was \[15,**14**,13,12\].

// **Constraints:**

// *   `3 <= arr.length <= 1000`
// *   `0 <= arr[i] <= 10^5`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Audible](https://leetcode.ca/tags/#Audible)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        for w in arr.windows(3) {
            if w[0] - w[1] != w[1] - w[2] {
                return if (w[0] - w[1]).abs() > (w[1] - w[2]).abs() {
                    w[0] + w[2] - w[1]
                } else {
                    w[2] + w[0] - w[1]
                };
            }
        }
        -1
    }
}


// impl Solution {
//     pub fn missing_number(arr: Vec<i32>) -> i32 {
//         (arr[0] + arr[arr.len()-1]) * (arr.len() as i32 + 1) / 2 - arr.iter().sum::<i32>()
//     }
// }
#[cfg(test)]
mod test {
    use super::*;
// [0,0,0,0,0]
// 输出：
// -1
// 预期结果：
// 0
    #[test]
    pub fn test_missing_number_1() {
        assert_eq!(9, Solution::missing_number(vec![5, 7, 11, 13]));
    }
    #[test]
    pub fn test_missing_number_2() {
        assert_eq!(14, Solution::missing_number(vec![15, 13, 12]));
    }
}
