// # [2638. Count the Number of K-Free Subsets](https://leetcode.com/problems/count-the-number-of-k-free-subsets)

// ## Description

//  You are given an integer array  nums , which contains  distinct  elements and an integer  k .

//  A subset is called a  k-Free  subset if it contains  no  two elements with an absolute difference equal to  k .
// Notice that the empty set is a  k-Free  subset.

//  Return  the number of  k-Free  subsets of   nums .

//  A  subset  of an array is a selection of elements (possibly none) of the array.

//   ### Example 1:

//  Input:  nums = [5,4,6], k = 1
//  Output:  5
//  Explanation:  There are 5 valid subsets: {}, {5}, {4}, {6} and {4, 6}.

//   ### Example 2:

//  Input:  nums = [2,3,5,8], k = 5
//  Output:  12
//  Explanation:  There are 12 valid subsets: {}, {2}, {3}, {5}, {8}, {2, 3}, {2, 3, 5}, {2, 5}, {2, 5, 8}, {2, 8}, {3, 5} and {5, 8}.

//   ### Example 3:

//  Input:  nums = [10,5,9,11], k = 20
//  Output:  16
//  Explanation:  All subsets are valid. Since the total count of subsets is 2 4  = 16, so the answer is 16.

//   Constraints:

// 	  1  <= nums.length  <= 50
// 	  1  <= nums[i]  <= 1000
// 	  1  <= k  <= 1000

// ## Solutions

// **Approach 1: Grouping + Dynamic Programming**

// First, sort the array $nums$ in ascending order,
// and then group the elements in the array according to the remainder modulo $k$,
// that is, the elements $nums[i] \bmod k$ with the same remainder are in the same group.
// Then for any two elements in different groups, their absolute difference is not equal to $k$.
// Therefore, we can obtain the number of subsets in each group,
// and then multiply the number of subsets in each group to obtain the answer.

// For each group $arr$, we can use dynamic programming to obtain the number of subsets.
// Let $f[i]$ denote the number of subsets of the first $i$ elements, and initially $f[0] = 1$, and $f[1]=2$.
// When $i \geq 2$, if $arr[i-1]-arr[i-2]=k$, if we choose $arr[i-1]$, then $f[i]=f[i-2]$;
// If we do not choose $arr[i-1]$, then $f[i]=f[i-1]$. Therefore, when $arr[i-1]-arr[i-2]=k$, we have $f[i]=f[i-1]+f[i-2]$;
// otherwise $f[i] = f[i - 1] \times 2$. The number of subsets of this group is $f[m]$, where $m$ is the length of the array $arr$.

// Finally, we multiply the number of subsets of each group to obtain the answer.

// The time complexity is $O(n \times \log n)$ and the space complexity is $O(n)$, where $n$ is the length of the array $nums$.

// ### **C++**

// ```cpp
// class Solution {
// public:
//     long long count_the_num_of_k_free_subsets(vector & nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_the_num_of_k_free_subsets(mut nums: Vec<i32>, k: i32) -> i64 {
        nums.sort_unstable();
        let mut g = std::collections::HashMap::new();
        for &x in &nums {
            g.entry(x % k).or_insert(Vec::new()).push(x);
        }
        let mut ans = 1;
        for arr in g.values() {
            let m = arr.len();
            let mut f = vec![0; m + 1];
            f[0] = 1;
            f[1] = 2;
            for i in 2..=m {
                f[i] = if arr[i - 1] - arr[i - 2] == k {
                    f[i - 1] + f[i - 2]
                } else {
                    f[i - 1] * 2
                };
            }
            ans *= f[m];
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_the_num_of_k_free_subsets_1() {
        assert_eq!(
            5,
            Solution::count_the_num_of_k_free_subsets(vec![5, 4, 6], 1)
        );
    }
    #[test]
    pub fn test_count_the_num_of_k_free_subsets_2() {
        assert_eq!(
            12,
            Solution::count_the_num_of_k_free_subsets(vec![2, 3, 5, 8], 5)
        );
    }
    #[test]
    pub fn test_count_the_num_of_k_free_subsets_3() {
        assert_eq!(
            16,
            Solution::count_the_num_of_k_free_subsets(vec![10, 5, 9, 11], 20)
        );
    }
}
