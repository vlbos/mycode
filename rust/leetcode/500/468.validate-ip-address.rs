/*
 * @lc app=leetcode id=468 lang=rust
 *
 * [468] Validate IP Address
 */

// @lc code=start
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let x: &[_] = &[':', '.'];
        if query_ip.find(x).is_none() {
            return "Neither".to_string();
        }
        if query_ip.find('.').is_some() {
            let ipv4 = query_ip.split('.').collect::<Vec<&str>>();
            if ipv4.len() != 4 {
                return "Neither".to_string();
            }
            for s in &ipv4 {
                if s.len() > 1 && &s[..1] == "0" {
                    return "Neither".to_string();
                }
                if let Ok(d) = s.parse::<i32>() {
                    if d < 0 || d > 255 {
                        return "Neither".to_string();
                    }
                } else {
                    return "Neither".to_string();
                }
            }
            return "IPv4".to_string();
        }
        let ipv6 = query_ip.split(':').collect::<Vec<&str>>();
        if ipv6.len() != 8 {
            return "Neither".to_string();
        }
        for s in &ipv6 {
            if s.is_empty() || s.len()>4 || s.chars().filter(char::is_ascii_alphanumeric).count() != s.len() || s.chars().filter(|c| c.is_ascii_alphabetic() && ((c.is_ascii_uppercase() && *c as u8 > b'F') ||(c.is_ascii_lowercase() && *c as u8 > b'f'))).count()>0{
                return "Neither".to_string();
            }
        }
        "IPv6".to_string()
    }
}
// @lc code=end
