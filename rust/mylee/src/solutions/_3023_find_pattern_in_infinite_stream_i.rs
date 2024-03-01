// # [3023. Find Pattern in Infinite Stream I](https://leetcode.com/problems/find-pattern-in-infinite-stream-i)

// ## Description

// You are given a binary array pattern and an object stream of class InfiniteStream representing a 0-indexed infinite stream of bits.

// The class InfiniteStream contains the following function:

// 	int next(): Reads a single bit (which is either 0 or 1) from the stream and returns it.

// Return the first starting index where the pattern matches the bits read from the stream.
// For example, if the pattern is [1, 0], the first match is the highlighted part in the stream [0, 1, 0, 1, ...].

//
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

//
// Constraints:

// 	1  <= pattern.length  <= 100
// 	pattern consists only of 0 and 1.
// 	stream consists only of 0 and 1.
// 	The input is generated such that the pattern 's start index exists in the first 105 bits of the stream.

//     int find_pattern(InfiniteStream* stream, vector<int>& pattern) {
//
#[allow(dead_code)]
pub struct InfiniteStream;
impl InfiniteStream {
    pub fn next() -> i32 {
        0
    }
}
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_pattern(stream: InfiniteStream, pattern: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_find_pattern_1() {
        assert_eq!(3, Solution::find_pattern(InfiniteStream, vec![0, 1]));
    }
    #[test]
    pub fn test_find_pattern_2() {
        assert_eq!(0, Solution::find_pattern(InfiniteStream, vec![0]));
    }
    #[test]
    pub fn test_find_pattern_3() {
        assert_eq!(2, Solution::find_pattern(InfiniteStream, vec![1, 1, 0, 1]));
    }
}
