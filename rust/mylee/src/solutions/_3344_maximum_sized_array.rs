// [3344. Maximum Sized Array 🔒](https://leetcode.com/problems/maximum-sized-array)
// =================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// Given a positive integer `s`, let `A` be a 3D array of dimensions `n × n × n`, where each element `A[i][j][k]` is defined as:

// *   `A[i][j][k] = i * (j OR k)`, where `0 <= i, j, k < n`.

// Return the **maximum** possible value of `n` such that the **sum** of all elements in array `A` does not exceed `s`.

// **Example 1:**

// **Input:** s = 10

// **Output:** 2

// **Explanation:**

// *   Elements of the array `A` for `n = 2`**:**
//     *   `A[0][0][0] = 0 * (0 OR 0) = 0`
//     *   `A[0][0][1] = 0 * (0 OR 1) = 0`
//     *   `A[0][1][0] = 0 * (1 OR 0) = 0`
//     *   `A[0][1][1] = 0 * (1 OR 1) = 0`
//     *   `A[1][0][0] = 1 * (0 OR 0) = 0`
//     *   `A[1][0][1] = 1 * (0 OR 1) = 1`
//     *   `A[1][1][0] = 1 * (1 OR 0) = 1`
//     *   `A[1][1][1] = 1 * (1 OR 1) = 1`
// *   The total sum of the elements in array `A` is 3, which does not exceed 10, so the maximum possible value of `n` is 2.

// **Example 2:**

// **Input:** s = 0

// **Output:** 1

// **Explanation:**

// *   Elements of the array `A` for `n = 1`:
//     *   `A[0][0][0] = 0 * (0 OR 0) = 0`
// *   The total sum of the elements in array `A` is 0, which does not exceed 0, so the maximum possible value of `n` is 1.

// **Constraints:**

// *   `0 <= s <= 1015` 

// int max_sized_array(long long s)



#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_sized_array(s: i64) -> i32 {
        use std::sync::OnceLock;
        static FF:OnceLock<Vec<i64>>=OnceLock::new();
        FF.get_or_init(||{
            let  mx=1330;
            let mut tmp=vec![0;mx];
            for i in 1..mx{
                tmp[i]=tmp[i-1]+i as i64;
            for j in 0..i{
                tmp[i]+=2*(i|j) as i64;
            }}
            tmp
        });
        let (mut l,mut r)=(1,1330);
        let f=FF.get().unwrap();
        while l<r{
            let m=(l+r+1)/2;
            if f[m as usize-1]*(m-1)*m/2<=s{
                l=m;
            }else{
                r=m-1;
            }
        }
        l as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_sized_array_1() {
        assert_eq!(2, Solution::max_sized_array(10));
    }
    #[test]
    pub fn test_max_sized_array_2() {
        assert_eq!(1, Solution::max_sized_array(0));
    }

}
