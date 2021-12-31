/*
 * @lc app=leetcode id=842 lang=rust
 *
 * [842] Split Array into Fibonacci Sequence
 */

// @lc code=start
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut list = Vec::new();
        fn back_tracking(
            list: &mut Vec<i32>,
            num: &String,
            index: usize,
            sum: i64,
            prev: i64,
        ) -> bool {
            if index == num.len() {
                return list.len() >= 3;
            }
            let mut curr = 0i64;
            for i in index..num.len() {
                let n = num.bytes().nth(i).unwrap();
                if i > index && num.bytes().nth(index).unwrap() == b'0' {
                    break;
                }
                curr = curr * 10 + (n - b'0') as i64;
                if curr > i32::MAX as i64 {
                    break;
                }
                if list.len() >= 2 {
                    if curr < sum {
                        continue;
                    } else if curr > sum {
                        break;
                    }
                }
                list.push(curr as i32);
                
                if back_tracking(list, num, i + 1, prev + curr, curr) {
                    return true;
                } else {
                    list.pop();
                }
            }
            false
        }
        back_tracking(&mut list, &num, 0, 0, 0);
        list
    }
}
// @lc code=end
