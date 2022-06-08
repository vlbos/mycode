/*
 * @lc app=leetcode id=1889 lang=rust
 *
 * [1889] Minimum Space Wasted From Packaging
 */

// @lc code=start
impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        let n = packages.len();
        let mut packages = packages;
        packages.sort();
        let mut pre = vec![0];
        let mut sum = 0;
        for &p in &packages {
            sum += p as i64;
            pre.push(sum);
        }
        let get = |left: usize, right| pre[right + 1] - pre[left];
        let mut ans = i64::MAX;
        let mut boxes=boxes;
        for bx in &mut boxes {
            bx.sort();
            if packages[packages.len() - 1] > bx[bx.len() - 1] {
                continue;
            }
            let mut pt = 0;
            let mut total = 0;
            for &mut y in bx {
                if y < packages[pt] {
                    continue;
                }
                let pt_next = packages[pt..].partition_point(|x| *x <= y) - 1;
                total += ((pt_next+ 1) as i64) * (y as i64) - get(pt, pt+pt_next);
                pt += pt_next + 1;
                if pt == packages.len() {
                    break;
                }
            }
            ans = ans.min(total);
        }
        if ans==i64::MAX{-1}else {(ans%1_000_000_007) as _}
    }
}
// @lc code=end
