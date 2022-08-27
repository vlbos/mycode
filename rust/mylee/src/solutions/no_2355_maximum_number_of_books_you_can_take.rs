// # [2355. Maximum Number of Books You Can Take](https://leetcode.com/problems/maximum-number-of-books-you-can-take)

// [中文文档](/solution/2300-2399/2355.Maximum%20Number%20of%20Books%20You%20Can%20Take/README.md)

// ## Description

// <p>You are given a <strong>0-indexed</strong> integer array <code>books</code> of length <code>n</code> where <code>books[i]</code> denotes the number of books on the <code>i<sup>th</sup></code> shelf of a bookshelf.</p>

// <p>You are going to take books from a <strong>contiguous</strong> section of the bookshelf spanning from <code>l</code> to <code>r</code> where <code>0 &lt;= l &lt;= r &lt; n</code>. For each index <code>i</code> in the range <code>l &lt;= i &lt; r</code>, you must take <strong>strictly fewer</strong> books from shelf <code>i</code> than shelf <code>i + 1</code>.</p>

// <p>Return <em>the <strong>maximum</strong> number of books you can take from the bookshelf.</em></p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> books = [8,5,2,7,9]
// <strong>Output:</strong> 19
// <strong>Explanation:</strong>
// - Take 1 book from shelf 1.
// - Take 2 books from shelf 2.
// - Take 7 books from shelf 3.
// - Take 9 books from shelf 4.
// You have taken 19 books, so return 19.
// It can be proven that 19 is the maximum number of books you can take.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> books = [7,0,3,4,5]
// <strong>Output:</strong> 12
// <strong>Explanation:</strong>
// - Take 3 books from shelf 2.
// - Take 4 books from shelf 3.
// - Take 5 books from shelf 4.
// You have taken 12 books so return 12.
// It can be proven that 12 is the maximum number of books you can take.
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> books = [8,2,3,7,3,4,0,1,4,3]
// <strong>Output:</strong> 13
// <strong>Explanation:</strong>
// - Take 1 book from shelf 0.
// - Take 2 books from shelf 1.
// - Take 3 books from shelf 2.
// - Take 7 books from shelf 3.
// You have taken 13 books so return 13.
// It can be proven that 13 is the maximum number of books you can take.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= books.length &lt;= 10<sup>5</sup></code></li>
// 	<li><code>0 &lt;= books[i] &lt;= 10<sup>5</sup></code></li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def maximumBooks(self, books: List[int]) -> int:

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
