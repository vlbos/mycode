// # [3037. Find Pattern in Infinite Stream II](https://leetcode.com/problems/find-pattern-in-infinite-stream-ii)

// <!-- tags:Array,String Matching,Sliding Window,Hash Function,Rolling Hash -->

// ## Description

// You are given a binary array pattern and an object stream of class InfiniteStream representing a 0-indexed infinite stream of bits.

// The class InfiniteStream contains the following function:

// 	int next(): Reads a single bit (which is either 0 or 1) from the stream and returns it.

// Return the first starting index where the pattern matches the bits read from the stream.
//  For example, if the pattern is [1, 0], the first match is the highlighted part in the stream [0, 1, 0, 1, ...].

// Example 1:

// Input: stream = [1,1,1,0,1,1,1,...], pattern = [0,1]
// Output: 3
// Explanation: The first occurrence of the pattern [0,1] is highlighted in the stream [1,1,1,0,1,...],
// which starts at index 3.

// Example 2:

// Input: stream = [0,0,0,0,...], pattern = [0]
// Output: 0
// Explanation: The first occurrence of the pattern [0] is highlighted in the stream [0,...],
//  which starts at index 0.

// Example 3:

// Input: stream = [1,0,1,1,0,1,1,0,1,...], pattern = [1,1,0,1]
// Output: 2
// Explanation: The first occurrence of the pattern [1,1,0,1] is highlighted in the stream [1,0,1,1,0,1,...],
// which starts at index 2.

// Constraints:

// 	1  <= pattern.length  <= 104
// 	pattern consists only of 0 and 1.
// 	stream consists only of 0 and 1.
// 	The input is generated such that the pattern 's start index exists in the first 105 bits of the stream.

#[allow(dead_code)]
pub struct InfiniteStream {
    s: Vec<u8>,
    i: usize,
}
impl InfiniteStream {
    pub fn new(s: Vec<u8>) -> Self {
        Self { s, i: 0 }
    }
    pub fn next(&mut self) -> i64 {
        self.i += 1;
        self.s[self.i - 1] as _
    }
}

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_pattern(mut stream: InfiniteStream, pattern: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        let m = pattern.len();
        let half = m >> 1;
        let (mask1, mask2) = ((1 << half) - 1, (1 << (m - half)) - 1);
        for i in 0..half {
            a |= (pattern[i] as i64) << (half - 1 - i);
        }
        for i in half..m {
            b |= (pattern[i] as i64) << (m - 1 - i);
        }
        let (mut x, mut y) = (0, 0);
        for i in 1.. {
            let mut v = stream.next();
            y = y << 1 | v;
            v = y >> (m - half) & 1;
            y &= mask2;
            x = x << 1 | v;
            x &= mask1;
            if i >= m && a == x && b == y {
                return (i - m) as _;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_find_pattern_1() {
        let is = InfiniteStream::new(vec![1, 1, 1, 0, 1, 1, 1]);
        assert_eq!(3, Solution::find_pattern(is, vec![0, 1]));
    }
    #[test]
    pub fn test_find_pattern_2() {
        let is = InfiniteStream::new(vec![0, 0, 0, 0]);
        assert_eq!(0, Solution::find_pattern(is, vec![0]));
    }
    #[test]
    pub fn test_find_pattern_3() {
        let is = InfiniteStream::new(vec![1, 0, 1, 1, 0, 1, 1, 0, 1]);
        assert_eq!(2, Solution::find_pattern(is, vec![1, 1, 0, 1]));
    }
}
