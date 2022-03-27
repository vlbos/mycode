/*
 * @lc app=leetcode id=1387 lang=rust
 *
 * [1387] Sort Integers by The Power Value
 */

// @lc code=start
impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
       let mut v:Vec<i32> = (lo..=hi).collect();
        use std::collections::HashMap;
        let mut m = HashMap::new();
        fn get_power(x: i32, m: &mut HashMap<i32, i32>) -> i32 {
            if x == 1 {
                return 0;
            }
            if let Some(&i) = m.get(&x) {
                return i;
            }

            let ans = get_power(if x % 2 == 0 { x / 2 } else { x * 3 + 1 }, m)+1;
            m.insert(x, ans);
            ans
        }
        v.sort_by_key(|x| (get_power(*x, &mut m), *x));
        v[k as usize - 1]
    }
}
// @lc code=end
