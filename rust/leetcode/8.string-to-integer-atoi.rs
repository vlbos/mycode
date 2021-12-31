/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut signed = 0;
        let mut ans = 0;
        let mut flag = false;
        for c in s.chars() {
            if c != ' ' && c != '+' && c != '-' && !c.is_ascii_digit() {
                if ans == 0 {
                    return 0;
                } else {
                    break;
                }
            }
            if c == '-' {
                if signed != 0 {
                    return if signed<0 {-ans}else{ans};
                }
                 if flag {
                    return if signed<0 {-ans}else{ans};
                }
                signed = -1;
            }
            if c == '+' {
                if  signed != 0 {
                    return if signed<0 {-ans}else{ans};
                }
                if flag {
                    return if signed<0 {-ans}else{ans};
                }
                signed = 1;
            }
            if c==' '{
                if  signed != 0 {
                    return if signed<0 {-ans}else{ans};
                }
                if flag {
                    return if signed<0 {-ans}else{ans};
                }
            }
            if c.is_ascii_digit() {
                flag = true;
                if signed >= 0 {
                    if ans > i32::MAX / 10 {
                        return i32::MAX;
                    }
                } else {
                    if ans * (-1) < i32::MIN / 10 {
                        return i32::MIN;
                    }
                }
                ans *= 10;
                let cc = (c as u8 - '0' as u8) as i32;
                if signed >= 0 {
                    if ans > i32::MAX - cc {
                        return i32::MAX;
                    }
                } else {
                    if ans * (-1) < i32::MIN + cc {
                        return i32::MIN;
                    }
                }
                ans += cc;
            }
        }
        if signed == 0 {
            signed = 1;
        }
        ans * signed
    }
}
// @lc code=end
