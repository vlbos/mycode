/*
 * @lc app=leetcode id=2040 lang=rust
 *
 * [2040] Kth Smallest Product of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let mut sums = vec![0; 200_005];
        for &num in &nums2 {
            sums[num as usize + 100_000] += 1;
        }
        for i in 1..sums.len() {
            sums[i] += sums[i - 1];
        }
        let sum = |x: i64| {
            if x < -100_000 {
                return 0;
            }
            if x > 100_000 {
                return sums[200_000];
            }
            sums[x as usize + 100_000]
        };
        let floor_div = |mut x: i64, mut y: i64| {
            if y < 0 {
                x = -x;
                y = -y;
            }
            if x < 0 {
                (x - y + 1) / y
            } else {
                x / y
            }
        };
        let ceil_div = |mut x: i64, mut y: i64| {
            if y < 0 {
                x = -x;
                y = -y;
            }
            if x < 0 {
                x / y
            } else {
                (x + y - 1) / y
            }
        };
        let (mut l, mut r) = (-10i64.pow(10), 10i64.pow(10));
        while l < r {
            let m = l+(r-l) / 2;
            let mut cnt = 0;
            for &v in &nums1 {
                let v = v as i64;
                if v < 0 {
                    cnt += sum(100_000) - sum(ceil_div(m, v) - 1);
                }
                if v == 0 && m >= 0 {
                    cnt += nums2.len() as i64;
                }
                if v > 0 {
                    cnt += sum(floor_div(m, v));
                }
            }
            if cnt < k {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}
// @lc code=end
