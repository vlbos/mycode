// 1243\. Array Transformation
// ===========================

// Given an initial array `arr`, every day you produce a new array using the array of the previous day.

// On the `i`\-th day, you do the following operations on the array of day `i-1` to produce the array of day `i`:

// 1.  If an element is smaller than both its left neighbor and its right neighbor, then this element is incremented.
// 2.  If an element is bigger than both its left neighbor and its right neighbor, then this element is decremented.
// 3.  The first and last elements never change.

// After some days, the array does not change. Return that final array.

// **Example 1:**

// **Input:** arr = \[6,2,3,4\]
// **Output:** \[6,3,3,4\]
// **Explanation:**
// On the first day, the array is changed from \[6,2,3,4\] to \[6,3,3,4\].
// No more operations can be done to this array.

// **Example 2:**

// **Input:** arr = \[1,6,3,4,3,5\]
// **Output:** \[1,4,4,4,4,5\]
// **Explanation:**
// On the first day, the array is changed from \[1,6,3,4,3,5\] to \[1,5,4,3,4,5\].
// On the second day, the array is changed from \[1,5,4,3,4,5\] to \[1,4,4,4,4,5\].
// No more operations can be done to this array.

// **Constraints:**

// *   `1 <= arr.length <= 100`
// *   `1 <= arr[i] <= 100`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Virtu Financial](https://leetcode.ca/tags/#Virtu%20Financial)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn transform_array(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        loop {
            let mut next = arr.clone();
            for i in 1..arr.len() - 1 {
                if arr[i - 1] > arr[i] && arr[i + 1] > arr[i] {
                    next[i] += 1;
                } else if arr[i - 1] < arr[i] && arr[i + 1] < arr[i] {
                    next[i] -= 1;
                }
            }
            if next == arr {
                break;
            }
            arr = next;
        }

        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_transform_array_1() {
        assert_eq!(
            vec![6, 3, 3, 4],
            Solution::transform_array(vec![6, 2, 3, 4])
        );
    }
    #[test]
    pub fn test_transform_array_2() {
        assert_eq!(
            vec![1, 4, 4, 4, 4, 5],
            Solution::transform_array(vec![1, 6, 3, 4, 3, 5])
        );
    }
}
