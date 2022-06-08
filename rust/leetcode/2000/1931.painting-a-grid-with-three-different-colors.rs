/*
 * @lc app=leetcode id=1931 lang=rust
 *
 * [1931] Painting a Grid With Three Different Colors
 */

// @lc code=start
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
         let mut valid = std::collections::HashMap::new();
        let mask_end = 3i32.pow(m as u32);
        for mask in 0..mask_end {
            let mut color = Vec::new();
            let mut mm = mask;
            for _ in 0..m {
                color.push(mm % 3);
                mm /= 3;
            }
            if color.windows(2).all(|x| x[0] != x[1]) {
                valid.insert(mask, color);
            }
        }
        let mut adjacent = std::collections::HashMap::new();
        for (mask1, color1) in &valid {
            for (mask2, color2) in &valid {
                if color1.iter().zip(color2).all(|x| x.0 != x.1) {
                    adjacent.entry(mask1).or_insert(Vec::new()).push(mask2);
                }
            }
        }
        let mut f = vec![0; mask_end as usize];
        for &mask in valid.keys() {
            f[mask as usize] = 1;
        }
        let p = 1_000_000_007;
        for _ in 1..n {
            let mut g = vec![0; mask_end as usize];
            for &mask2 in valid.keys() {
                for &mask1 in adjacent.get(&mask2).unwrap_or(&Vec::new()) {
                    g[mask2 as usize] += f[*mask1 as usize];
                    if g[mask2 as usize] >= p {
                        g[mask2 as usize] -= p;
                    }
                }
            }
            f = g;
        }
        let mut ans = 0;
        for &num in &f {
            ans += num;
            if ans >= p {
                ans -= p;
            }
        }
        ans
    }
}
// @lc code=end
