// 1213\. Intersection of Three Sorted Arrays
// ==========================================

// Given three integer arrays `arr1`, `arr2` and `arr3` **sorted** in **strictly increasing** order,
//  return a sorted array of **only** the integers that appeared in **all** three arrays.

// **Example 1:**

// **Input:** arr1 = \[1,2,3,4,5\], arr2 = \[1,2,5,7,9\], arr3 = \[1,3,4,5,8\]
// **Output:** \[1,5\]
// **Explanation:** Only 1 and 5 appeared in the three arrays.

// **Constraints:**

// *   `1 <= arr1.length, arr2.length, arr3.length <= 1000`
// *   `1 <= arr1[i], arr2[i], arr3[i] <= 2000`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [TripAdvisor](https://leetcode.ca/tags/#TripAdvisor)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let (mut i2, mut i3) = (0, 0);
        for &v in &arr1 {
            while i2 < arr2.len() && v > arr2[i2] {
                i2 += 1;
            }
            if i2 == arr2.len() || v < arr2[i2] {
                continue;
            }
            while i3 < arr3.len() && v > arr3[i3] {
                i3 += 1;
            }
            if i3 < arr3.len() && v == arr3[i3] {
                ans.push(v);
            }
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_arrays_intersection_1() {
        assert_eq!(
            vec![1, 5],
            Solution::arrays_intersection(
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 5, 7, 9],
                vec![1, 3, 4, 5, 8]
            )
        );
    }
}
