/*
 * @lc app=leetcode id=420 lang=rust
 *
 * [420] Strong Password Checker
 */

// @lc code=start
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len() as i32;
        let mut categories_array = vec![0; 3];
        for c in password.chars() {
            if c.is_ascii_lowercase() {
                categories_array[0] = 1;
            } else if c.is_ascii_uppercase() {
                categories_array[1] = 1;
            } else if c.is_ascii_digit() {
                categories_array[2] = 1;
            }
        }
        let categories = categories_array.iter().sum::<i32>();
        if n < 6 {
            return (6 - n).max(3 - categories);
        }
        if n <= 20 {
            let mut replace = 0;
            let mut cnt = 0;
            let mut cur = '#';
            for c in password.chars() {
                if c == cur {
                    cnt += 1;
                } else {
                    replace += cnt / 3;
                    cnt = 1;
                    cur = c;
                }
            }
            replace += cnt / 3;
            return replace.max(3 - categories);
        }
        let mut replace = 0;
        let mut remove = n - 20;
        let mut rm2 = 0;
        let mut cnt = 0;
        let mut cur = '#';
        for c in password.chars() {
            if c == cur {
                cnt += 1;
            } else {
                if remove > 0 && cnt >= 3 {
                    if cnt % 3 == 0 {
                        remove -= 1;
                        replace -= 1;
                    } else if cnt % 3 == 1 {
                        rm2 += 1;
                    }
                }
                replace += cnt / 3;
                cnt = 1;
                cur = c;
            }
        }
        if remove > 0 && cnt >= 3 {
            if cnt % 3 == 0 {
                remove -= 1;
                replace -= 1;
            } else if cnt % 3 == 1 {
                rm2 += 1;
            }
        }
        replace += cnt / 3;
        let use2 = replace.min(rm2.min(remove / 2));
        replace -= use2;
        remove -= use2 * 2;
        let use3 = replace.min(remove / 3);
        replace -= use3;
        remove -= use3 * 3;
        n - 20 + replace.max(3 - categories)
    }
}
// @lc code=end
