// 248\. Strobogrammatic Number III
// ================================

// A strobogrammatic number is a number that looks the same when rotated 180 degrees (looked at upside down).

// Write a function to count the total strobogrammatic numbers that exist in the range of low <= num <= high.

// **Example:**

// **Input:** low = "50", high = "100"
// **Output:** 3
// **Explanation:** 69, 88, and 96 are three strobogrammatic numbers.

// **Note:**
// Because the range might be a large number, the _low_ and _high_ numbers are represented as string.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)
 
#[allow(dead_code)] 
 struct Solution;
// @lc code=start

// use std::cell::RefCell;
// use std::rc::Rc;

// const CENTER_CHAR_SIZE: usize = 3;
// const PAIR_CHAR_SIZE: usize = 5;
// const CENTER_CHARS: [u64; CENTER_CHAR_SIZE] = [0, 1, 8];
// const PAIR_CHARS: [(u64, u64); PAIR_CHAR_SIZE] = [(0, 0), (1, 1), (6, 9), (8, 8), (9, 6)];

impl Solution {
    pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
        // let ll = low.len();
        // let lh = high.len();
        // let low = low.parse().unwrap();
        // let high = high.parse().unwrap();
        // let res = Rc::new(RefCell::new(vec![]));
        // for l in ll..=lh {
        //     let target = if (l & 1) == 1 { l / 2 } else { l / 2 - 1 } as u32;
        //     Solution::strobogrammatic_in_range_rec(
        //         low,
        //         high,
        //         0,
        //         0,
        //         target,
        //         (l - 1) as u32,
        //         res.clone(),
        //     );
        // }
        // let res_b = res.borrow();
        // res_b.len() as i32

        let mut ans = 0;
        for i in low.len()..=high.len() {
            let digits = if i % 2 == 0 {
                vec![""]
            } else {
                vec!["0", "1", "8"]
            };
            for d in &digits {
                find(&low, &high, d.to_string(), i, &mut ans);
            }
        }
        fn find(low: &String, high: &String, s: String, len: usize, ans: &mut i32) {
            if len == s.len() {
                if len == low.len() && s < *low || len == high.len() && s > *high {
                    return;
                }
                *ans += 1;
                return;
            }
            for (j, d) in [['0', '0'], ['1', '1'], ['8', '8'], ['6', '9'], ['9', '6']]
                .iter()
                .enumerate()
            {
                if j == 0 && s.len() + 2 >= len {
                    continue;
                }
                find(low, high, format!("{}{}{}", d[0], s, d[1]), len, ans);
            }
        }

        ans
    }

    // pub fn strobogrammatic_in_range_rec(
    //     low: u64,
    //     high: u64,
    //     sum: u64,
    //     curr: u32,
    //     target: u32,
    //     all: u32,
    //     res: Rc<RefCell<Vec<String>>>,
    // ) {
    //     if curr < target || ((curr == target) && ((all & 1) == 1)) {
    //         for (a, b) in &PAIR_CHARS {
    //             let high_order_factor = u64::pow(10, all - curr);
    //             let low_order_factor = u64::pow(10, curr);
    //             let new_sum = sum + (*a) * high_order_factor + (*b) * low_order_factor;
    //             if new_sum > high || new_sum + high_order_factor <= low {
    //                 continue;
    //             }
    //             Solution::strobogrammatic_in_range_rec(
    //                 low,
    //                 high,
    //                 new_sum,
    //                 curr + 1,
    //                 target,
    //                 all,
    //                 res.clone(),
    //             );
    //         }
    //     } else if (curr == target) && ((all & 1) == 0) {
    //         for a in &CENTER_CHARS {
    //             let new_sum = sum + a * u64::pow(10, all - curr);
    //             if new_sum > high || new_sum < low {
    //                 continue;
    //             }
    //             Solution::strobogrammatic_in_range_rec(
    //                 low,
    //                 high,
    //                 new_sum,
    //                 curr + 1,
    //                 target,
    //                 all,
    //                 res.clone(),
    //             );
    //         }
    //     } else if sum == 0 || sum >= u64::pow(10, all) {
    //         let mut res_bm = res.borrow_mut();
    //         if sum == 0 && !res_bm.is_empty() && res_bm[0] == "0" {
    //             return;
    //         }
    //         res_bm.push(sum.to_string());
    //     }
    // }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strobogrammatic_in_range_1() {
        let src = Solution::strobogrammatic_in_range(String::from("0"), String::from("100"));
        assert_eq!(src, 7);
    }

    #[test]
    fn test_strobogrammatic_in_range_2() {
        let src = Solution::strobogrammatic_in_range(String::from("50"), String::from("100"));
        assert_eq!(src, 3);
    }
}
