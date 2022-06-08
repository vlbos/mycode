/*
 * @lc app=leetcode id=480 lang=rust
 *
 * [480] Sliding Window Median
 */

// @lc code=start
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
       use std::collections::BinaryHeap;
        use std::collections::HashMap;
        let (mut small, mut large) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut small_len, mut large_len) = (0, 0);
        let mut delayed = HashMap::new();
        let k = k as usize;
        let prune =
            |heap: &mut BinaryHeap<i64>, is_small: bool, delayed: &mut HashMap<i64, i32>| {
                let mut num = 0;
                while !heap.is_empty() {
                    num = *heap.peek().unwrap();
                    if !is_small {
                        num = -num;
                    }
                    if !delayed.contains_key(&num) {
                        break;
                    }
                    delayed.entry(num).and_modify(|x| *x -= 1);
                    if *delayed.get(&num).unwrap() == 0 {
                        delayed.remove(&num);
                    }
                    heap.pop();
                }
            };
        let make_balance = |small: &mut BinaryHeap<i64>,
                            large: &mut BinaryHeap<i64>,
                            small_len: &mut i32,
                            large_len: &mut i32,
                            delayed: &mut HashMap<i64, i32>| {
            if *small_len > *large_len + 1 {
                large.push(-*small.peek().unwrap());
                small.pop();
                *large_len += 1;
                *small_len -= 1;
                prune(small, true, delayed);
            } else if *small_len < *large_len {
                small.push(-*large.peek().unwrap());
                large.pop();
                *large_len -= 1;
                *small_len += 1;
                prune(large, false, delayed);
            }
        };
        let insert = |num: i64,
                      small: &mut BinaryHeap<i64>,
                      large: &mut BinaryHeap<i64>,
                      small_len: &mut i32,
                      large_len: &mut i32,
                      delayed: &mut HashMap<i64, i32>| {
            if small.is_empty() || num <= *small.peek().unwrap() {
                small.push(num);
                *small_len += 1;
            } else {
                large.push(-num);
                *large_len += 1;
            }
            make_balance(small, large, small_len, large_len, delayed);
        };
        let erase = |num: i64,
                     small: &mut BinaryHeap<i64>,
                     large: &mut BinaryHeap<i64>,
                     small_len: &mut i32,
                     large_len: &mut i32,
                     delayed: &mut HashMap<i64, i32>| {
            *delayed.entry(num).or_insert(0) += 1;
            if num <= *small.peek().unwrap() {
                *small_len -= 1;
                if num == *small.peek().unwrap() {
                    prune(small, true, delayed);
                }
            } else {
                *large_len -= 1;
                if num == -*large.peek().unwrap() {
                    prune(large, false, delayed);
                }
            }
            make_balance(small, large, small_len, large_len, delayed);
        };
        let get_median = |small: &BinaryHeap<i64>, large: &BinaryHeap<i64>| -> f64 {
            if k % 2 > 0 {
                *small.peek().unwrap() as _
            } else {
                (*small.peek().unwrap() - *large.peek().unwrap() ) as f64 / 2.0
            }
        };
        for &num in &nums[0..k] {
            insert(
                num as i64,
                &mut small,
                &mut large,
                &mut small_len,
                &mut large_len,
                &mut delayed,
            );
        }
        let mut ans = vec![get_median(&small, &large)];
        for i in k..nums.len() {
            insert(
                nums[i] as i64,
                &mut small,
                &mut large,
                &mut small_len,
                &mut large_len,
                &mut delayed,
            );
            erase(
                nums[i - k] as i64,
                &mut small,
                &mut large,
                &mut small_len,
                &mut large_len,
                &mut delayed,
            );
            ans.push(get_median(&small, &large));
        }
        ans
    }
}
// @lc code=end
