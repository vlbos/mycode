// # [2386. Find the K-Sum of an Array](https://leetcode.com/problems/find-the-k-sum-of-an-array)

// [中文文档](/solution/2300-2399/2386.Find%20the%20K-Sum%20of%20an%20Array/README.md)

// ## Description

// <p>You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>. You can choose any <strong>subsequence</strong> of the array and sum all of its elements together.</p>

// <p>We define the <strong>K-Sum</strong> of the array as the <code>k<sup>th</sup></code> <strong>largest</strong> subsequence sum that can be obtained (<strong>not</strong> necessarily distinct).</p>

// <p>Return <em>the K-Sum of the array</em>.</p>

// <p>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</p>

// <p><strong>Note</strong> that the empty subsequence is considered to have a sum of <code>0</code>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [2,4,-2], k = 5
// <strong>Output:</strong> 2
// <strong>Explanation:</strong> All the possible subsequence sums that we can obtain are the following sorted in decreasing order:
// - 6, 4, 4, 2, <u>2</u>, 0, 0, -2.
// The 5-Sum of the array is 2.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> nums = [1,-2,3,4,-10,12], k = 16
// <strong>Output:</strong> 10
// <strong>Explanation:</strong> The 16-Sum of the array is 10.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>n == nums.length</code></li>
// 	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
// 	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
// 	<li><code>1 &lt;= k &lt;= min(2000, 2<sup>n</sup>)</code></li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def kSum(self, nums: List[int], k: int) -> int:



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
