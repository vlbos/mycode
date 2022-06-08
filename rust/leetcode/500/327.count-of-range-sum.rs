/*
 * @lc app=leetcode id=327 lang=rust
 *
 * [327] Count of Range Sum
 */

// @lc code=start
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
       let mut s = 0;
        let mut sum = vec![0];
        for &num in &nums {
            s += num as i64;
            sum.push(s);
        }
        fn count_range_sum_recursive(
            sum: &mut Vec<i64>,
            lower: i64,
            upper: i64,
            left: usize,
            right: usize,
        ) -> i32 {
            if left == right {
                return 0;
            }
            let mid = (left + right) / 2;
            let n1 = count_range_sum_recursive(sum, lower, upper, left, mid);
            let n2 = count_range_sum_recursive(sum, lower, upper, mid + 1, right);
            let mut ans = n1 + n2;
            let mut i = left;
            let (mut l, mut r) = (mid + 1, mid + 1);
            while i <= mid {
                while l <= right && sum[l] - sum[i] < lower {
                    l += 1;
                }
                while r <= right && sum[r] - sum[i] <= upper {
                    r += 1;
                }
                ans += (r - l) as i32;
                i += 1;
            }
            let mut sorted = vec![0; right - left + 1];
            let (mut p1, mut p2) = (left, mid + 1);
            let mut p = 0;
            while p1 <= mid || p2 <= right {
                if p1 > mid {
                    sorted[p] = sum[p2];
                    p2 += 1;
                } else if p2 > right {
                    sorted[p] = sum[p1];
                    p1 += 1;
                } else {
                    if sum[p1] < sum[p2] {
                        sorted[p] = sum[p1];
                        p1 += 1;
                    } else {
                        sorted[p] = sum[p2];
                        p2 += 1;
                    }
                }
                p += 1;
            }
            for (i, &v) in sorted.iter().enumerate() {
                sum[left + i] = v;
            }
            ans
        }
        let n = sum.len();
        count_range_sum_recursive(&mut sum, lower as i64, upper as i64, 0, n - 1)
    }
}
// @lc code=end
