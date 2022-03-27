/*
 * @lc app=leetcode id=1898 lang=rust
 *
 * [1898] Maximum Number of Removable Characters
 */

// @lc code=start
impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let (ns, np) = (s.len(), p.len());
        let (bs, bp) = (s.as_bytes(), p.as_bytes());
        let n = removable.len();
        let check = |k: usize| -> bool {
            let mut state = vec![true; ns];
            for i in 0..k {
                state[removable[i] as usize] = false;
            }
            let mut j = 0;
            for i in 0..ns {
                if state[i] && bs[i] == bp[j] {
                    j += 1;
                    if j == np {
                        return true;
                    }
                }
            }
            false
        };
        let (mut l, mut r) = (0, n + 1);
        while l < r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32- 1
    }
}
// @lc code=end
