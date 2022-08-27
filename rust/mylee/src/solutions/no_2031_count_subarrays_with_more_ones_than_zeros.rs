// [2031\. Count Subarrays With More Ones Than Zeros (Medium)](https://leetcode.com/problems/count-subarrays-with-more-ones-than-zeros/)[](https://leetcode.ca/2021-10-15-2031-Count-Subarrays-With-More-Ones-Than-Zeros/#2031-count-subarrays-with-more-ones-than-zeros-medium)
// =============================================================================================================================================================================================================================================================================

// You are given a binary array `nums` containing only the integers `0` and `1`. Return _the number of **subarrays** in nums that have **more**_ `1`'_s than_ `0`_'s. Since the answer may be very large, return it **modulo**_ `109 + 7`.

// A **subarray** is a contiguous sequence of elements within an array.

// **Example 1:**

// **Input:** nums = \[0,1,1,0,1\]
// **Output:** 9
// **Explanation:**
// The subarrays of size 1 that have more ones than zeros are: \[1\], \[1\], \[1\]
// The subarrays of size 2 that have more ones than zeros are: \[1,1\]
// The subarrays of size 3 that have more ones than zeros are: \[0,1,1\], \[1,1,0\], \[1,0,1\]
// The subarrays of size 4 that have more ones than zeros are: \[1,1,0,1\]
// The subarrays of size 5 that have more ones than zeros are: \[0,1,1,0,1\]

// **Example 2:**

// **Input:** nums = \[0\]
// **Output:** 0
// **Explanation:**
// No subarrays have more ones than zeros.

// **Example 3:**

// **Input:** nums = \[1\]
// **Output:** 1
// **Explanation:**
// The subarrays of size 1 that have more ones than zeros are: \[1\]

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `0 <= nums[i] <= 1`

// **Companies**:  
// [Google](https://leetcode.com/company/google)

// **Related Topics**:  
// [Array](https://leetcode.com/tag/array/), [Binary Search](https://leetcode.com/tag/binary-search/), [Divide and Conquer](https://leetcode.com/tag/divide-and-conquer/), [Binary Indexed Tree](https://leetcode.com/tag/binary-indexed-tree/), [Segment Tree](https://leetcode.com/tag/segment-tree/), [Merge Sort](https://leetcode.com/tag/merge-sort/), [Ordered Set](https://leetcode.com/tag/ordered-set/)

// **Similar Questions**:

// *   [Ones and Zeroes (Medium)](https://leetcode.com/problems/ones-and-zeroes/)
// *   [Longer Contiguous Segments of Ones than Zeros (Easy)](https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/)

// Solution 1.[](https://leetcode.ca/2021-10-15-2031-Count-Subarrays-With-More-Ones-Than-Zeros/#solution-1)
// --------------------------------------------------------------------------------------------------------

//     index:     0  1  2  3  4
//     A:         0  1  1  0  1
//     Diff:   0 -1  0  1  0  1 // Count(1) - Count(0)
//     Count:     0 +1 +3 +1 +4 // Sum of number of diffs less than the current diff.
    

// Let `diff[i]` be the count of `1`s minus count of `0`s before `A[i]` inclusive.

// For each `A[i]`, it will add “sum of number of diffs less than the current diff” to the answer.

// So we need a data structure with which we can query a range sum. Segment tree and Binary Indexed Tree are good for this purpose.

// This implementation uses BIT.

//     // OJ: https://leetcode.com/problems/count-subarrays-with-more-ones-than-zeros/
//     // Time: O(NlogN)
//     // Space: O(N)
//     // Ref: https://leetcode.com/problems/count-subarrays-with-more-ones-than-zeros/discuss/1512961/BIT-vs.-O(n)
//     const int N = 200000, mod = 1e9 + 7;
//     int bt[N + 1] = {};
//     class Solution {
//         int sum(int i) {
//             int ans = 0;
//             for (++i; i > 0; i -= i & -i) ans += bt[i];
//             return ans;
//         }
//         void update(int i, int val) {
//             for (++i; i <= N; i += i & -i) bt[i] += val;
//         }
//     public:
//         int subarraysWithMoreZerosThanOnes(vector<int>& A) {



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
