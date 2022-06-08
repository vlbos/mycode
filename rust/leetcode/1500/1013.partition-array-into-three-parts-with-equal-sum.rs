/*
 * @lc app=leetcode id=1013 lang=rust
 *
 * [1013] Partition Array Into Three Parts With Equal Sum
 */

// @lc code=start
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = arr.len() - 1;
        let mut sum = arr.iter().sum::<i32>();
        if sum % 3 != 0 {
            return false;
        }
        let mut a: i32 = sum / 3;
        let mut s = 0;
        let mut n = arr.len();
        while i < n {
            s += arr[i];
            if s == a {
                break;
            }
            i += 1;
        }
        if s != a {
            return false;
        }
        j = i + 1;
        while j+1 < n {
            s += arr[j];
            if s == a * 2 {
                println!("{}",j);
                return true;
            }
            j += 1;
        }
        false
    }
}
// @lc code=end
