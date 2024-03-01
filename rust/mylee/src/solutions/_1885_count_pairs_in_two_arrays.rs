// 1885\. Count Pairs in Two Arrays[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#1885-count-pairs-in-two-arrays)
// =================================================================================================================================

// Level[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#level)
// -----------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#description)
// -----------------------------------------------------------------------------------------

// Given two integer arrays `nums1` and `nums2` of length `n`, count the pairs of indices `(i, j)` such that `i < j` and `nums1[i] + nums1[j] > nums2[i] + nums2[j]`.

// Return _the **number of pairs** satisfying the condition_.

// **Example 1:**

// **Input:** nums1 = \[2,1,2,1\], nums2 = \[1,2,1,2\]

// **Output:** 1

// **Explanation:** The pairs satisfying the condition are:

// *   (0, 2) where 2 + 2 > 1 + 1.

// **Example 2:**

// **Input:** nums1 = \[1,10,6,2\], nums2 = \[1,4,1,5\]

// **Output:** 5

// **Explanation:** The pairs satisfying the condition are:

// *   (0, 1) where 1 + 10 > 1 + 4.
// *   (0, 2) where 1 + 6 > 1 + 1.
// *   (1, 2) where 10 + 6 > 4 + 1.
// *   (1, 3) where 10 + 2 > 4 + 5.
// *   (2, 3) where 6 + 2 > 1 + 5.

// **Constraints:**

// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10^5`
// *   `1 <= nums1[i], nums2[i] <= 10^5`

// Solution[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#solution)
// -----------------------------------------------------------------------------------

// Create an array `differences` with length `n` such that `differences[i] = nums1[i] - nums2[i]` for all `0 <= i < n`. Sort `differences`. For each `0 <= i < n - 1`, find the minimum `index` such that `differences[index] + differences[i] > 0`, or `index = n` if `differences[n - 1] + differences[i] <= 0`, and the number of `j`’s for the current `i` is `n - index`. In this way, the number of pairs of indices `(i, j)` can be calculated.

//     class Solution {
//         public long count_pairs(int[] nums1, int[] nums2) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut diff: Vec<i32> = nums1
            .into_iter()
            .zip(nums2)
            .map(|(v1, v2)| v1 - v2)
            .collect();
        diff.sort();
        let mut ans = 0;
        let n = diff.len();
        for i in 0..n - 1 {
            let target = -diff[i] + 1;
            let mut index = i + 1;
            let (Ok(j) | Err(j)) = diff[i + 1..].binary_search(&target);
            index += j;
            ans += n - index;
        }
        ans as _
    }
}

// impl Solution {
//     pub fn count_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//  let n = nums1.len();
//         let mut nums = vec![0;n];

//         for i in 0..n {
//             nums[i] = nums1[i] - nums2[i];
//         }

//         nums.sort();

//         let mut ans = 0;

//         for i in 0..n-1 {
//             let mut lo = i + 1;
//             let mut hi = n - 1;
//             let mut pos = n;

//             while lo <= hi && hi < n {
//                 let mid = lo + (hi - lo) / 2;

//                 if nums[i] + nums[mid] > 0 {
//                     pos = mid;
//                     hi = mid - 1;
//                 } else {
//                     lo = mid + 1;
//                 }
//             }

//             ans += n - pos;
//         }

//         ans as i64
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    // [5,1,1,15,3,14,19,1,9,12,6,8,2,4,19,17,19,5]
    // [1,16,5,3,7,9,19,3,7,2,13,4,4,17,13,12,19,16]
    // 输出：
    // 70
    // 预期结果：
    // 71

    #[test]
    pub fn test_count_pairs_1() {
        assert_eq!(1, Solution::count_pairs(vec![2, 1, 2, 1], vec![1, 2, 1, 2]));
    }
    #[test]
    pub fn test_count_pairs_2() {
        assert_eq!(
            5,
            Solution::count_pairs(vec![1, 10, 6, 2], vec![1, 4, 1, 5])
        );
    }
}
