// 1868\. Product of Two Run-Length Encoded Arrays[](https://leetcode.ca/2021-07-17-1868-Product-of-Two-Run-Length-Encoded-Arrays/#1868-product-of-two-run-length-encoded-arrays)
// ==============================================================================================================================================================================

// Level[](https://leetcode.ca/2021-07-17-1868-Product-of-Two-Run-Length-Encoded-Arrays/#level)
// --------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-17-1868-Product-of-Two-Run-Length-Encoded-Arrays/#description)
// --------------------------------------------------------------------------------------------------------

// **Run-length encoding** is a compression algorithm that allows for an integer array `nums` with many segments of **consecutive repeated** numbers to be represented by a (generally smaller) 2D array `encoded`. Each `encoded[i] = [val_i, freq_i]` describes the `i-th` segment of repeated numbers in nums where `val_i` is the value that is repeated `freq_i` times.

// *   For example, `nums = [1,1,1,2,2,2,2,2]` is represented by the **run-length encoded** array `encoded = [[1,3],[2,5]]`. Another way to read this is “three `1`s followed by five `2`s”.

// The **product** of two run-length encoded arrays `encoded1` and `encoded2` can be calculated using the following steps:

// 1.  **Expand** both `encoded1` and `encoded2` into the full arrays `nums1` and `nums2` respectively.
// 2.  Create a new array `prodNums` of length `nums1.length` and set `prodNums[i] = nums1[i] * nums2[i]`.
// 3.  **Compress** `prodNums` into a run-length encoded array and return it.

// You are given two **run-length encoded** arrays `encoded1` and `encoded2` representing full arrays `nums1` and `nums2` respectively. Both `nums1` and `nums2` have the **same length**. Each `encoded1[i] = [val_i, freq_i]` describes the `i-th` segment of `nums1`, and each `encoded2[j] = [val_j, freq_j]` describes the `j-th` segment of `nums2`.

// Return _the **product** of `encoded1` and `encoded2`_.

// **Note:** Compression should be done such that the run-length encoded array has the minimum possible length.

// **Example 1:**

// **Input:** encoded1 = \[\[1,3\],\[2,3\]\], encoded2 = \[\[6,3\],\[3,3\]\]

// **Output:** \[\[6,6\]\]

// **Explanation:** encoded1 expands to \[1,1,1,2,2,2\] and encoded2 expands to \[6,6,6,3,3,3\].

// prodNums = \[6,6,6,6,6,6\], which is compressed into the run-length encoded array \[\[6,6\]\].

// **Example 2:**

// **Input:** encoded1 = \[\[1,3\],\[2,1\],\[3,2\]\], encoded2 = \[\[2,3\],\[3,3\]\]

// **Output:** \[\[2,3\],\[6,1\],\[9,2\]\]

// **Explanation:** encoded1 expands to \[1,1,1,2,3,3\] and encoded2 expands to \[2,2,2,3,3,3\].

// prodNums = \[2,2,2,6,9,9\], which is compressed into the run-length encoded array \[\[2,3\],\[6,1\],\[9,2\]\].

// **Constraints:**

// *   `1 <= encoded1.length, encoded2.length <= 10^5`
// *   `encoded1[i].length == 2`
// *   `encoded2[j].length == 2`
// *   `1 <= val_i, freq_i <= 10^4 for each encoded1[i]`.
// *   `1 <= val_j, freq_j <= 10^4 for each encoded2[j]`.
// *   The full arrays that `encoded1` and `encoded2` represent are the same length.

//    List<List<Integer>> find_rle_array(int[][] encoded1, int[][] encoded2)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn find_rle_array(encoded1: Vec<Vec<i32>>, encoded2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let e1: Vec<i32> = encoded1
            .into_iter()
            .map(|x| vec![x[0]; x[1] as usize])
            .flatten()
            .collect();
        let e2: Vec<i32> = encoded2
            .into_iter()
            .map(|x| vec![x[0]; x[1] as usize])
            .flatten()
            .collect();
        let e: Vec<i32> = e1.into_iter().zip(e2).map(|(v1, v2)| v1 * v2).collect();
        let mut ans = Vec::new();
        let mut pre = e[0];
        let mut cnt = 0;
        for &v in &e {
            if pre == v {
                cnt += 1;
            } else {
                ans.push(vec![pre, cnt]);
                pre = v;
                cnt = 1;
            }
        }
        if cnt > 0 {
            ans.push(vec![pre, cnt]);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_rle_array_1() {
        assert_eq!(
            vec![vec![6, 6]],
            Solution::find_rle_array(vec![vec![1, 3], vec![2, 3]], vec![vec![6, 3], vec![3, 3]])
        );
    }
    #[test]
    pub fn test_find_rle_array_2() {
        assert_eq!(
            vec![vec![2, 3], vec![6, 1], vec![9, 2]],
            Solution::find_rle_array(
                vec![vec![1, 3], vec![2, 1], vec![3, 2]],
                vec![vec![2, 3], vec![3, 3]]
            )
        );
    }
}
