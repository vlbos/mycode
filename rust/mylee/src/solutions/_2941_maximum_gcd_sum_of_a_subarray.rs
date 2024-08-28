// # [2941. Maximum GCD-Sum of a Subarray](https://leetcode.com/problems/maximum-gcd-sum-of-a-subarray)

// ## Description

// You are given an array of integers nums and an integer k.

// The gcd-sum of an array a is calculated as follows:

// 	Let s be the sum of all the elements of a.
// 	Let g be the greatest common divisor of all the elements of a.
// 	The gcd-sum of a is equal to s * g.

// Return the maximum gcd-sum of a subarray of nums with at least k elements.

//
// Example 1:

// Input: nums = [2,1,4,4,4,2], k = 2
// Output: 48
// Explanation: We take the subarray [4,4,4], the gcd-sum of this array is 4 * (4 + 4 + 4) = 48.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 48.

// Example 2:

// Input: nums = [7,3,9,4], k = 1
// Output: 81
// Explanation: We take the subarray [9], the gcd-sum of this array is 9 * 9 = 81.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 81.

//
// Constraints:

// 	n == nums.length
// 	1  <= n  <= 105
// 	1  <= nums[i]  <= 106
// 	1  <= k  <= n

//     long long max_gcd_sum(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_gcd_sum(nums: Vec<i32>, k: i32) -> i64 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut s: Vec<_> = nums
            .iter()
            .scan(0, |sum, &x| {
                *sum += x as i64;
                Some(*sum)
            })
            .collect();
        s.insert(0, 0);
        let mut f = vec![];
        let mut ans = 0;
        for (i, v) in nums.into_iter().enumerate() {
            let mut g: Vec<(usize, i32)> = vec![];
            for &(j, x) in &f {
                let y = gcd(x, v);
                if g.is_empty() || g.last().unwrap().1 != y {
                    g.push((j, y));
                }
            }
            f = g;
            f.push((i, v));
            for &(j, x) in &f {
                if i - j + 1 >= k as usize {
                    ans = ans.max((s[i + 1] - s[j]) * x as i64);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // nums =
    // [353520,526320,418320,405360,299520,504720,273600,349440,238140,439488,302400,290160,459360,420480,433440,488880,712320,335520,609840,483840,700560,422640,338688,553680,255600,163440,252720,160560,669600,668880,251280,972720,443520,564480,494640,560160,348480,300960,596880,126000,640080,932400,560880,787500,552960,527040,519120,442800,395280,804384,317520,275760,276480,286560,362160,453600,703440,614160,478080,185760,675360,262800,320880,725760,745920,293760,197280,707760,349200,326880,452340,599760,643104,172620,766080,680400,158400,841680,267840,553680,694800,312480,362160,504000,153720,201600,568800,400680,150480,579600,553680,430920,851760,756000,603360,480960,546840,559440,156240,267120,406080,259920,376560,254520,612000,444960,344880,347760,658560,490320,602640,504000,574560,244080,576000,213840,345744,677520,581040,397440,536400,414000,176400,658080,383040,312480,391860,210240,645120,673200,501120,579600,213120,367920,524160,730800,601920,231840,153360,699120,120960,289440,47712...
    // View less
    // k =
    // 2634

    // Use Testcase
    // Output
    // 4294087860
    // Expected
    // 4586235960
    #[test]
    pub fn test_max_gcd_sum_1() {
        assert_eq!(48, Solution::max_gcd_sum(vec![2, 1, 4, 4, 4, 2], 2));
    }
    #[test]
    pub fn test_max_gcd_sum_2() {
        assert_eq!(81, Solution::max_gcd_sum(vec![7, 3, 9, 4], 1));
    }
}
