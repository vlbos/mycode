/*
 * @lc app=leetcode id=963 lang=rust
 *
 * [963] Minimum Area Rectangle II
 */

// @lc code=start
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let p = points
            .iter().cloned()
            .collect::<std::collections::HashSet<Vec<i32>>>();
        let mut ans = f64::MAX;

        let mut area = |p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>| -> f64 {
            let d12 = (p2[0] - p1[0]) * (p2[0] - p1[0]) + (p2[1] - p1[1]) * (p2[1] - p1[1]);
            let d13 = (p3[0] - p1[0]) * (p3[0] - p1[0]) + (p3[1] - p1[1]) * (p3[1] - p1[1]);
            (d12 as f64).sqrt() * (d13 as f64).sqrt()
        };
        for i in 0..n {
            let p1 = &points[i];
            for j in 0..n {
                if i == j {
                    continue;
                }
                let p2 = &points[j];
                for k in j + 1..n {
                    if i == k {
                        continue;
                    }
                    let p3 = &points[k];
                    let p4 = vec![p2[0] + p3[0] - p1[0], p2[1] + p3[1] - p1[1]];
                    if p.contains(&p4) {
                        let dot =
                            (p2[0] - p1[0]) * (p3[0] - p1[0]) + (p2[1] - p1[1]) * (p3[1] - p1[1]);
                        if dot == 0 {
                            ans = ans.min(area(&p1, &p2, &p3));
                        }
                    }
                }
            }
        }
        if ans<f64::MAX {ans}else{0f64}
    }
}
// @lc code=end
