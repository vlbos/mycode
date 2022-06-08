/*
 * @lc app=leetcode id=1300 lang=rust
 *
 * [1300] Sum of Mutated Array Closest to Target
 */

// @lc code=start
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let mut pre = vec![0; n + 1];
        for i in 1..=n {
            pre[i] = pre[i - 1] + arr[i - 1];
        }
        let mut r = *arr.iter().max().unwrap();
        let mut l = 0;
        let mut ans = -1;
        while l <= r {
            let mid = (l + r) / 2;
            if let Ok(i) | Err(i) = arr.binary_search(&mid) {
                let cur = pre[i] + (n - i) as i32 * mid ;
                if cur  <= target {
                    ans = mid ;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        let check = |x: i32| -> i32 {
            let mut ans = 0;
            for &v in &arr {
                ans += if v >= x { x } else { v };
            }
            ans
        };
        let small = check(ans);
        let big = check(ans + 1);
        if (small - target).abs() <= (big - target).abs() {
            ans
        } else {
            ans + 1
        }
    }
}
// @lc code=end
