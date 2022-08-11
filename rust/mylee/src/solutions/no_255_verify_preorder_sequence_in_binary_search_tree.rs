// 255\. Verify Preorder Sequence in Binary Search Tree
// ====================================================

// Given an array of numbers, verify whether it is the correct preorder traversal sequence of a binary search tree.

// You may assume each number in the sequence is unique.

// Consider the following binary search tree:

//      5
//     / \\
//    2   6
//   / \\
//  1   3

// **Example 1:**

// **Input:** \[5,2,6,1,3\]
// **Output:** false

// **Example 2:**

// **Input:** \[5,2,1,3,6\]
// **Output:** true

// **Follow up:**
// Could you do it using only constant space complexity?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Mathworks](https://leetcode.ca/tags/#Mathworks) [Uber](https://leetcode.ca/tags/#Uber) [Walmart Labs](https://leetcode.ca/tags/#Walmart%20Labs) [Zenefits](https://leetcode.ca/tags/#Zenefits)
#[allow(dead_code)]
pub struct Solution;
// @lc code=start

impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        fn verify_preorder(arr: &[i32]) -> bool {
            if arr.is_empty() {
                return true;
            }
            let root = arr[0];
            let i = 1 + arr[1..]
                .iter()
                .position(|&x| x >= root)
                .unwrap_or(arr.len() - 1);
            let left_size = i;
            if arr[i..].iter().any(|x| *x < root) {
                return false;
            }
            verify_preorder(&arr[1..left_size]) && verify_preorder(&arr[left_size..])
        }
        verify_preorder(&preorder)
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_preorder_1() {
        let src = vec![5, 2, 6, 1, 3];
        assert!(!Solution::verify_preorder(src));
    }

    #[test]
    fn test_verify_preorder_2() {
        let src = vec![5, 2, 1, 3, 6];
        assert!(Solution::verify_preorder(src));
    }
}
