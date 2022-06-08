/*
 * @lc app=leetcode id=805 lang=rust
 *
 * [805] Split Array With Same Average
 */

// @lc code=start
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return false;
        }
        let sum = nums.iter().sum::<i32>();
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let nn = n as i32;
        let g = gcd(nn, sum);
        let mu = vec![-(sum / g), nn / g];
        let frac_add = |a: &Vec<i32>, b: &Vec<i32>| -> Vec<i32> {
            let (mut numer, mut denom) = (a[0] * b[1] + a[1] * b[0], a[1] * b[1]);
            let g = gcd(numer, denom);
            numer /= g;
            denom /= g;
            if denom < 0 {
                numer *= -1;
                denom *= -1;
            }
            vec![numer, denom]
        };
        let mut nums2 = nums
            .iter()
            .map(|x| frac_add(&vec![*x, 1], &mu))
            .collect::<Vec<Vec<i32>>>();
        use std::collections::HashSet;
        let mut left = HashSet::new();
        left.insert(nums2[0].clone());
        for i in 1..n / 2 {
            let mut left2 = HashSet::new();
            left2.insert(nums2[i].clone());
            for p in &left {
                left2.insert(p.clone());
                left2.insert(frac_add(p, &nums2[i]));
            }
            left = left2;
        }
        if left.contains(&vec![0, 1]) {
            return true;
        }
        let mut right = HashSet::new();
        right.insert(nums2[n - 1].clone());
        for i in n / 2..n - 1 {
            let mut right2 = HashSet::new();
            right2.insert(nums2[i].clone());
            for p in &right {
                right2.insert(p.clone());
                right2.insert(frac_add(p, &nums2[i]));
            }
            right = right2;
        }
        if right.contains(&vec![0, 1]) {
            return true;
        }
        let mut sleft = vec![0, 1];
        for i in 0..n / 2 {
            sleft = frac_add(&sleft, &nums2[i]);
        }
        let mut sright = vec![0, 1];
        for i in n / 2..n {
            sright = frac_add(&sright, &nums2[i]);
        }
        for ha in &left {
            let ha2 = vec![-ha[0], ha[1]];
            if right.contains(&ha2) && (ha.clone() != sleft || ha2 != sright) {
                return true;
            }
        }
        false
    }
}
// @lc code=end
