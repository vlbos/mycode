// 774\. Minimize Max Distance to Gas Station
// ==========================================

// On a horizontal number line, we have gas stations at positions `stations[0], stations[1], ..., stations[N-1]`, where `N = stations.length`.

// Now, we add `K` more gas stations so that **D**, the maximum distance between adjacent gas stations, is minimized.

// Return the smallest possible value of **D**.

// **Example:**

// **Input:** stations = \[1, 2, 3, 4, 5, 6, 7, 8, 9, 10\], K = 9
// **Output:** 0.500000

// **Note:**

// 1.  `stations.length` will be an integer in range `[10, 2000]`.
// 2.  `stations[i]` will be an integer in range `[0, 10^8]`.
// 3.  `K` will be an integer in range `[1, 10^6]`.
// 4.  Answers within `10^-6` of the true value will be accepted as correct.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
#[allow(dead_code)]
    pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
        let (mut left, mut right) = (0.0, 1e8);
        while right - left > 1e-6 {
            let mid = (right + left) / 2.0;
            let mut cnt = 0.0;
            for s in stations.windows(2) {
                cnt += (s[1] - s[0]) as f64 / mid;
            }
            if cnt > k as f64 {
                left = mid;
            } else {
                right = mid;
            }
            println!("{},{}", left, right);
        }
        right / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minmax_gas_dist_1() {
        assert!(
            (0.500000 - Solution::minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9)).abs()
                < 1e-6
        );
    }
}
