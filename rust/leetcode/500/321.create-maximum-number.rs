/*
 * @lc app=leetcode id=321 lang=rust
 *
 * [321] Create Maximum Number
 */

// @lc code=start
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let (m, n) = (nums1.len(), nums2.len());
        let k = k as usize;
        let mut max_seq = vec![0; k];
        let (mut start, mut end) = (if k > n { k - n } else { 0 }, k.min(m));
        let get_max_seq = |nums: &Vec<i32>, k: usize| -> Vec<i32> {
            let mut s = Vec::new();
            let mut r = nums.len() - k;
            for (i, &num) in nums.iter().enumerate() {
                while !s.is_empty() && s[s.len() - 1] < num && r > 0 {
                    s.pop();
                    r -= 1;
                }
                if s.len() < k {
                    s.push(num);
                } else {
                    r -= 1;
                }
            }
            s
        };
         let compare =
            |sub_seq1: &Vec<i32>, mut i1: usize, sub_seq2: &Vec<i32>, mut i2: usize| -> bool {
                let (x, y) = (sub_seq1.len(), sub_seq2.len());
                while i1 < x && i2 < y {
                    let d = sub_seq1[i1] - sub_seq2[i2];
                    if d != 0 {
                        return d > 0;
                    }
                    i1 += 1;
                    i2 += 1;
                }
                x - i1 > y - i2
            };
        let merge = |sub_seq1: &Vec<i32>, sub_seq2: &Vec<i32>| -> Vec<i32> {
            let (x, y) = (sub_seq1.len(), sub_seq2.len());
            if x == 0 {
                return sub_seq2.to_vec();
            }
            if y == 0 {
                return sub_seq1.to_vec();
            }
            let merge_len = x + y;
            let mut merged= Vec::new();
            let (mut i1, mut i2) = (0, 0);
            for _ in 0..merge_len {
                if compare(sub_seq1, i1, sub_seq2, i2) {
                    merged.push(sub_seq1[i1]);
                    i1 += 1;
                } else {
                    merged.push(sub_seq2[i2]);
                    i2 += 1;
                }
            }
            merged
        };
       
        for i in start..=end {
            let sub_seq1 = get_max_seq(&nums1, i);
            let sub_seq2 = get_max_seq(&nums2, k - i);
            let cur_max_seq = merge(&sub_seq1, &sub_seq2);
            if compare(&cur_max_seq,0, &max_seq,0) {
                max_seq = cur_max_seq;
            }
        }
        max_seq
    }
}
// @lc code=end
