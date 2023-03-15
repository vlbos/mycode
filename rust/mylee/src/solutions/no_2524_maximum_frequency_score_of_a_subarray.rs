// # [2524. Maximum Frequency Score of a Subarray](https://leetcode.com/problems/maximum-frequency-score-of-a-subarray)
// ## Description

//  You are given an integer array  nums  and a  positive  integer  k .
//  The  frequency score  of an array is the sum of the  distinct  values in the array raised to the power of their  frequencies ,
// taking the sum  modulo   10^9  + 7 .
// 	 For example, the frequency score of the array  [5,4,5,7,4,4]  is  (4^3  + 5^2  + 7^1 ) modulo (10^9  + 7) = 96 .

//  Return  the  maximum  frequency score of a  subarray  of size   k   in   nums .
//  You should maximize the value under the modulo and not the actual value.

//  A  subarray  is a contiguous part of an array.

//  Example 1:

//  Input:  nums = [1,1,1,2,1,2], k = 3
//  Output:  5
//  Explanation:  The subarray [2,1,2] has a frequency score equal to 5.
//  It can be shown that it is the maximum frequency score we can have.

//  Example 2:

//  Input:  nums = [1,1,1,1,1,1], k = 4
//  Output:  1
//  Explanation:  All the subarrays of length 4 have a frequency score equal to 1.

//   Constraints:

// 	  1 <= k <= nums.length <= 10^5
// 	  1 <= nums[i] <= 10^6
//  int max_frequency_score(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_frequency_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let k = k as usize;
        for &num in &nums[..k] {
            *cnt.entry(num as i64).or_insert(0) += 1;
        }
        let mut cur = 0;
        let m = 1_000_000_007;
        let qmi = |mut a: i64, mut k: i64| {
            let mut ans = 1;
            while k > 0 {
                if k % 2 > 0 {
                    ans = ans * a % m;
                }
                k /= 2;
                a = a * a % m;
            }
            ans
        };
        for (&k, &v) in &cnt {
            cur = (cur + qmi(k as i64, v)) % m;
        }
        let mut ans = cur;
        for (i, &v) in nums[k..].iter().enumerate() {
            if nums[i] == v {
                continue;
            }
            let (a, b) = (nums[i] as i64, v as i64);
            cur += if *cnt.get(&b).unwrap_or(&0) == 0 {
                b
            } else {
                (b - 1) * qmi(b, cnt[&b]) % m
            };
            cur -= if cnt[&a] > 1 {
                (a - 1) * qmi(a, cnt[&a] - 1) % m
            } else {
                a
            };
            cur = (cur + m) % m;
            ans = ans.max(cur);
            *cnt.entry(b).or_insert(0) += 1;
            cnt.entry(a).and_modify(|x| {
                *x -= 1;
            });
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_frequency_score_1() {
        assert_eq!(5, Solution::max_frequency_score(vec![1, 1, 1, 2, 1, 2], 3));
    }
    #[test]
    pub fn test_max_frequency_score_2() {
        assert_eq!(1, Solution::max_frequency_score(vec![1, 1, 1, 1, 1, 1], 4));
    }
}
