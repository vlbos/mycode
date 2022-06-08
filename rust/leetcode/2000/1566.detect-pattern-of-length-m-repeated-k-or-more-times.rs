/*
 * @lc app=leetcode id=1566 lang=rust
 *
 * [1566] Detect Pattern of Length M Repeated K or More Times
 */

// @lc code=start
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        if arr.len() < m * k {
            return false;
        }
        let mut s = 0;
        let mut a = vec![0; m];
        for j in 0..=arr.len() - m * k {
            s=0;
            let mut i = j;
            while i <= arr.len() - m {
                let t = &arr[i..i + m];
                if a != t {
                    a = t.to_vec().clone();
                    s = 1;
                } else {
                    s += 1;
                    if s == k {
                        return true;
                    }
                }
                i += m;
            }
            if s == k {
                        return true;
            }
        }
        s == k
    }
}
// @lc code=end
