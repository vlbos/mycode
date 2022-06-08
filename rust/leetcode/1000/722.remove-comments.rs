/*
 * @lc app=leetcode id=722 lang=rust
 *
 * [722] Remove Comments
 */

// @lc code=start
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut buffer: String = String::new();
        let mut mode: bool = false;
        
        for line in source.iter() {
            let mut i = 0;
            let len = line.len();
            while i < len {
                if mode {
                    if i < len - 1 && &line[i..i+2] == "*/" {
                        mode = false;
                        i += 1;
                    }
                } else {
                    if i < len - 1 && &line[i..i+2] == "//" {
                        break;
                    } else if i < len - 1 && &line[i..i+2] == "/*" {
                        mode = true;
                        i += 1;
                    } else {
                        buffer.push_str(&line[i..i+1]);
                    }
                }
                i += 1;
            }
            if !mode && buffer.len() > 0 {
                res.push(buffer);
                buffer = String::new();
            }    
        }
        
        return res;
    }
}
// @lc code=end
