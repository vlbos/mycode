/*
 * @lc app=leetcode id=1095 lang=rust
 *
 * [1095] Find in Mountain Array
 */

// @lc code=start
/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
       let (mut l, mut r) = (0, mountainArr.length()-1);
        while l < r {
            let mid = (l + r) / 2;
            if mountainArr.get(mid) < mountainArr.get(mid + 1) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        let peak = l;
        let binary_search =
            |mut target: i32, mountainArr: &MountainArray, mut l: i32, mut r: i32, flag: bool| -> i32 {
                if flag {
                    target *= -1;
                }
                while l <= r {
                    let mid = (l + r) / 2;
                    let mut cur = mountainArr.get(mid);
                    if flag {
                        cur *= -1;
                    }
                    if cur == target {
                        return mid;
                    } else if cur < target {
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                                    -1
            };
        let index = binary_search(target, mountainArr, 0, peak, false);
        if index != -1 {
            index
        } else {
            binary_search(
                target,
                mountainArr,
                peak + 1,
                mountainArr.length() - 1,
                true,
            )
        }
    }
}
// @lc code=end
