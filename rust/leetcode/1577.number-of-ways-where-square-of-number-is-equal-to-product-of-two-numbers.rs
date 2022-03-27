/*
 * @lc app=leetcode id=1577 lang=rust
 *
 * [1577] Number of Ways Where Square of Number Is Equal to Product of Two Numbers
 */

// @lc code=start
impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let (mut n1, mut n2) = (HashMap::new(), HashMap::new());
        for &v in &nums1 {
            *n1.entry(v).or_insert(0) += 1;
        }
        for &v in &nums2 {
            *n2.entry(v).or_insert(0) += 1;
        }
        let get_triplets = |n1: &HashMap<i32, i32>, n2: &HashMap<i32, i32>| -> i32 {
            let mut ans = 0;
            for (&k1, &v1) in n1 {
                let k1 = k1 as i64;
                let kk = k1 * k1;
                for (&k2, &v2) in n2 {
                    let k2 = k2 as i64;
                    if kk % k2 == 0 {
                        let k3 = kk / k2;
                        let k33 = k3 as i32;
                        if k2 == k3 {
                            ans += v1 * v2 * (v2 - 1) / 2;
                        } else if k2 < k3 && n2.contains_key(&k33) {
                            let v3 = n2[&k33];
                            ans += v1 * v2 * v3;
                        }
                    }
                }
            }
            ans
        };
        get_triplets(&n1, &n2) + get_triplets(&n2, &n1)
    }
}
// @lc code=end
