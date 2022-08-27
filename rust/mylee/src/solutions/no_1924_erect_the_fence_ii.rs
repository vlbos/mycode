// 1924\. Erect the Fence II[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#1924-erect-the-fence-ii)
// ============================================================================================================

// Level[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#level)
// ----------------------------------------------------------------------

// Hard

// Description[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#description)
// ----------------------------------------------------------------------------------

// You are given a 2D integer array `trees` where `trees[i] = [x_i, y_i]` represents the location of the `i-th` tree in the garden.

// You are asked to fence the entire garden using the minimum length of rope possible. The garden is well-fenced only if **all the trees are enclosed** and the rope used **forms a perfect circle**. A tree is considered enclosed if it is inside or on the border of the circle.

// More formally, you must form a circle using the rope with a center `(x, y)` and radius `r` where all trees lie inside or on the circle and `r` is **minimum**.

// Return _the center and radius of the circle as a length 3 array `[x, y, r]`_. Answers within `10-5` of the actual answer will be accepted.

// **Example 1:**

// ![Image text](https://assets.leetcode.com/uploads/2021/07/06/trees1.png)

// **Input:** trees = \[\[1,1\],\[2,2\],\[2,0\],\[2,4\],\[3,3\],\[4,2\]\]

// **Output:** \[2.00000,2.00000,2.00000\]

// **Explanation:** The fence will have center = (2, 2) and radius = 2

// **Example 2:**

// ![Image text](https://assets.leetcode.com/uploads/2021/07/06/trees2.png)

// **Input:** trees = \[\[1,2\],\[2,2\],\[4,2\]\]

// **Output:** \[2.50000,2.00000,1.50000\]

// **Explanation:** The fence will have center = (2.5, 2) and radius = 1.5

// **Constraints:**

// *   `1 <= trees.length <= 3000`
// *   `trees[i].length == 2`
// *   `0 <= x_i, y_i <= 3000`

// Solution[](https://leetcode.ca/2021-08-03-1924-Erect-the-Fence-II/#solution)
// ----------------------------------------------------------------------------

// This problem is the smallest-circle problem. Use Welzl’s algorithm to find the smallest circle’s center and radius.

//     class Solution {
//         public double[] outerTrees(int[][] trees) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
