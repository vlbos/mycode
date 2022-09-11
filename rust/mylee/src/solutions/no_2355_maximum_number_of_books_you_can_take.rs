// # [2355. Maximum Number of Books You Can Take](https://leetcode.com/problems/maximum-number-of-books-you-can-take)

// ## Description

// You are given a 0-indexed integer array books of length n where books[i] denotes the number of books on the ith shelf of a bookshelf.

// You are going to take books from a contiguous section of the bookshelf spanning from l to r where 0 <= l <= r < n. For each index i in the range l <= i < r, you must take strictly fewer books from shelf i than shelf i + 1.

// Return the maximum number of books you can take from the bookshelf.

// Example 1:

//
// Input: books = [8,5,2,7,9]
// Output: 19
// Explanation:
// - Take 1 book from shelf 1.
// - Take 2 books from shelf 2.
// - Take 7 books from shelf 3.
// - Take 9 books from shelf 4.
// You have taken 19 books, so return 19.
// It can be proven that 19 is the maximum number of books you can take.
//

// Example 2:

//
// Input: books = [7,0,3,4,5]
// Output: 12
// Explanation:
// - Take 3 books from shelf 2.
// - Take 4 books from shelf 3.
// - Take 5 books from shelf 4.
// You have taken 12 books so return 12.
// It can be proven that 12 is the maximum number of books you can take.
//

// Example 3:

//
// Input: books = [8,2,3,7,3,4,0,1,4,3]
// Output: 13
// Explanation:
// - Take 1 book from shelf 0.
// - Take 2 books from shelf 1.
// - Take 3 books from shelf 2.
// - Take 7 books from shelf 3.
// You have taken 13 books so return 13.
// It can be proven that 13 is the maximum number of books you can take.
//

// Constraints:

//
// 	1 <= books.length <= 105
// 	0 <= books[i] <= 105
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def maximum_books(self, books: List[int]) -> int:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximum_books(books: Vec<i32>) -> i64 {
        let helper =
            |p: i64, c: i64| (p * (p + 1) - if p > c { (p - c) * (p - c + 1) } else { 0 }) / 2;
        let mut s = Vec::new();
        let mut ans = 0;
        let mut now = 0;
        for (i, &v) in books.iter().enumerate() {
            while !s.is_empty()
                && v - books[*s.last().unwrap()] < (i as i32 - *s.last().unwrap() as i32)
            {
                let j = s.pop().unwrap();
                now -= helper(
                    books[j] as i64,
                    if s.is_empty() {
                        j as i64 + 1
                    } else {
                        j as i64 - *s.last().unwrap() as i64
                    },
                );
            }
            now += helper(
                v as i64,
                if s.is_empty() {
                    i as i64 + 1
                } else {
                    i as i64 - *s.last().unwrap() as i64
                },
            );
            s.push(i);
            ans = ans.max(now);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_books_1() {
        assert_eq!(19, Solution::maximum_books(vec![8, 5, 2, 7, 9]));
    }
    #[test]
    pub fn test_maximum_books_2() {
        assert_eq!(12, Solution::maximum_books(vec![7, 0, 3, 4, 5]));
    }
    #[test]
    pub fn test_maximum_books_3() {
        assert_eq!(
            13,
            Solution::maximum_books(vec![8, 2, 3, 7, 3, 4, 0, 1, 4, 3])
        );
    }
}
