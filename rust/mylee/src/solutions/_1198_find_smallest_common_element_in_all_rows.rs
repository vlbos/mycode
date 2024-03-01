// 1198\. Find Smallest Common Element in All Rows
// ===============================================

// Given a matrix `mat` where every row is sorted in **increasing** order, return the **smallest common element** in all rows.

// If there is no common element, return `-1`.

// **Example 1:**

// **Input:** mat = \[\[1,2,3,4,5\],\[2,4,5,8,10\],\[3,5,7,9,11\],\[1,3,5,7,9\]\]
// **Output:** 5

// **Constraints:**

// *   `1 <= mat.length, mat[i].length <= 500`
// *   `1 <= mat[i][j] <= 10^4`
// *   `mat[i]` is sorted in increasing order.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Microsoft](https://leetcode.ca/tags/#Microsoft)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        for &v in &mat[0] {
            let mut found = true;
            for r in &mat[1..] {
                if r.binary_search(&v).is_err() {
                    found = false;
                    break;
                }
            }
            if found {
                return v;
            }
        }
        -1
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_smallest_common_element_1() {
        assert_eq!(
            5,
            Solution::smallest_common_element(vec![
                vec![1, 2, 3, 4, 5],
                vec![2, 4, 5, 8, 10],
                vec![3, 5, 7, 9, 11],
                vec![1, 3, 5, 7, 9]
            ])
        );
    }
}
