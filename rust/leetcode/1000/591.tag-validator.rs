/*
 * @lc app=leetcode id=591 lang=rust
 *
 * [591] Tag Validator
 */

// @lc code=start
impl Solution {
    pub fn is_valid(code: String) -> bool {
       let bc = code.as_bytes();
        if bc[0] != b'<' || bc[bc.len() - 1] != b'>' {
            return false;
        }
        let mut s = Vec::new();
        let mut contains_tag = false;
        let is_valid_tagname =
            |code: &[u8], s: &mut Vec<Vec<u8>>, contains_tag: &mut bool, ending: bool| {
                if code.len() < 1 || code.len() > 9 {
                    return false;
                }
                if code.iter().any(|x| !((*x )as char).is_ascii_uppercase()) {
                    return false;
                }
                if ending {
                    if !s.is_empty() && s[s.len() - 1] == code.to_vec() {
                        s.pop();
                    } else {
                        return false;
                    }
                } else {
                    *contains_tag = true;
                    s.push(code.to_vec());
                }
                true
            };
        let is_valid_cdata = |code: &[u8]| {
            String::from_utf8(code.to_vec())
                .unwrap()
                .find("[CDATA[")
                .unwrap_or(code.len())
                == 0
        };
        let mut i = 0;
        while i < bc.len() {
            let b = bc[i];
            let mut ending = false;
            let mut close_index = None;
            if s.is_empty() && contains_tag {
                return false;
            }
            if b == b'<' {
                if !s.is_empty() && bc[i + 1] == b'!' {
                    close_index = code[i + 1..].find("]]>");
                    if close_index.is_none() || !is_valid_cdata(&bc[i + 2..i + 1 + close_index.unwrap()]) {
                        return false;
                    }
                } else {
                    if bc[i + 1] == b'/' {
                        i += 1;
                        ending = true;
                    }
                    close_index = code[i + 1..].find(">");
                    if close_index.is_none()
                        || !is_valid_tagname(
                            &bc[i + 1..i + 1 + close_index.unwrap()],
                            &mut s,
                            &mut contains_tag,
                            ending,
                        )
                    {
                        return false;
                    }
                }
                i = close_index.unwrap() + i + 1;
            }
            i += 1;
        }
        s.is_empty() && contains_tag
    }
}
// @lc code=end
