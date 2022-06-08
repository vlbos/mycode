/*
 * @lc app=leetcode id=1053 lang=rust
 *
 * [1053] Previous Permutation With One Swap
 */

// @lc code=start
impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let mut max = i32::MIN;
        let mut k = 0;
        let mut flag = false;
        for i in (0..n - 1).rev() {
            if arr[i] > arr[i + 1] {
                for j in i + 1..n {
                    if arr[i] > arr[j] {
                        flag = true;
                        if arr[j] > max {
                            max = arr[j];
                            k = j;
                        }
                    }
                }
                if flag {
                    arr.swap(i, k);
                    break;
                }
            }
        }
        arr
    }
}
// @lc code=end
