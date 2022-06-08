/*
 * @lc app=leetcode id=1998 lang=rust
 *
 * [1998] GCD Sort of an Array
 */

// @lc code=start
impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let mut nums1 = nums.clone();
        let mut p: Vec<i32> = (0..300011).collect();
        fn find(x: i32, p: &mut Vec<i32>) -> i32 {
            if p[x as usize] == x {
                return x;
            }
            p[x as usize] = find(p[x as usize], p);
            p[x as usize]
        }
        let merge = |x: i32, y: i32, p: &mut Vec<i32>| {
            let (x, y) = (find(x, p), find(y, p));
            if x == y {
                return;
            }
            p[x as usize] = y;
        };
        for &num in &nums {
            let mut c = num;
            let mut i = 2;
            while i * i <= c {
                let mut flag = false;
                while c % i == 0 {
                    c /= i;
                    flag = true;
                }
                if flag {
                    merge(num, i,&mut p);
                }
                i += 1;
            }
            if c > 1 {
                merge(num, c,&mut p);
            }
        }
        nums1.sort();
        for (i, &num1) in nums1.iter().enumerate() {
            if num1 == nums[i] {
                continue;
            }
            if find(num1, &mut p) != find(nums[i], &mut p) {
                return false;
            }
        }
        true
    }
}
// @lc code=end
