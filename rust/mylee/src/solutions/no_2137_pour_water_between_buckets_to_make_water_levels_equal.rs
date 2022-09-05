// # [2137. Pour Water Between Buckets to Make Water Levels Equal](https://leetcode.com/problems/pour-water-between-buckets-to-make-water-levels-equal)

// ## Description

// You have n buckets each containing some gallons of water in it, represented by a 0-indexed integer array buckets,
//  where the ith bucket contains buckets[i] gallons of water. You are also given an integer loss.

// You want to make the amount of water in each bucket equal. You can pour any amount of water from one bucket to another bucket
//  (not necessarily an integer). However, every time you pour k gallons of water, you spill loss percent of k.

// Return the maximum amount of water in each bucket after making the amount of water equal.
//  Answers within 10-5 of the actual answer will be accepted.

// Example 1:

//
// Input: buckets = [1,2,7], loss = 80
// Output: 2.00000
// Explanation: Pour 5 gallons of water from buckets[2] to buckets[0].
// 5 * 80% = 4 gallons are spilled and buckets[0] only receives 5 - 4 = 1 gallon of water.
// All buckets have 2 gallons of water in them so return 2.
//

// Example 2:

//
// Input: buckets = [2,4,6], loss = 50
// Output: 3.50000
// Explanation: Pour 0.5 gallons of water from buckets[1] to buckets[0].
// 0.5 * 50% = 0.25 gallons are spilled and buckets[0] only receives 0.5 - 0.25 = 0.25 gallons of water.
// Now, buckets = [2.25, 3.5, 6].
// Pour 2.5 gallons of water from buckets[2] to buckets[0].
// 2.5 * 50% = 1.25 gallons are spilled and buckets[0] only receives 2.5 - 1.25 = 1.25 gallons of water.
// All buckets have 3.5 gallons of water in them so return 3.5.
//

// Example 3:

//
// Input: buckets = [3,3,3,3], loss = 40
// Output: 3.00000
// Explanation: All buckets already have the same amount of water in them.
//

// Constraints:

//
// 	1 <= buckets.length <= 105
// 	0 <= buckets[i] <= 105
// 	0 <= loss <= 99
//

//  double equalize_water(vector<int>& A, int loss) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn equalize_water(buckets: Vec<i32>, loss: i32) -> f64 {
        let rate = (100.0 - loss as f64) / 100.0;
        let check = |x: f64| {
            let (mut extra, mut need) = (0.0, 0.0);
            for &b in &buckets {
                let b = b as f64;
                if b >= x {
                    extra += b - x;
                } else {
                    need += x - b;
                }
            }
            extra * rate >= need
        };
        let (mut left, mut right) = (
            *buckets.iter().min().unwrap() as f64,
            buckets.iter().sum::<i32>() as f64 / buckets.len() as f64,
        );
        while left + 0.00001 <= right {
            let mid = (right + left) / 2.0;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        (right * 100000.0) as i64 as f64 / 100000.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_equalize_water_1() {
        assert_eq!(2.00000, Solution::equalize_water(vec![1, 2, 7], 80));
    }
    #[test]
    pub fn test_equalize_water_2() {
        assert_eq!(3.50000, Solution::equalize_water(vec![2, 4, 6], 50));
    }
    #[test]
    pub fn test_equalize_water_3() {
        assert_eq!(3.00000, Solution::equalize_water(vec![3, 3, 3, 3], 40));
    }
}
