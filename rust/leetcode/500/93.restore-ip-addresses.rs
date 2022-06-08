/*
 * @lc app=leetcode id=93 lang=rust
 *
 * [93] Restore IP Addresses
 */

// @lc code=start
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let is_component = |slice: &str| -> bool {
            if slice.starts_with('0') {
                slice.len() == 1
            } else {
                slice.parse::<u8>().is_ok()
            }
        };
        let mut result = Vec::new();

        for length_1 in 1.max(s.len().saturating_sub(9))..=3.min(s.len().saturating_sub(3)) {
            let (first, s) = s.split_at(length_1);

            if is_component(first) {
                for length_2 in 1.max(s.len().saturating_sub(6))..=3.min(s.len().saturating_sub(2))
                {
                    let (second, s) = s.split_at(length_2);

                    if is_component(second) {
                        for length_3 in
                            1.max(s.len().saturating_sub(3))..=3.min(s.len().saturating_sub(1))
                        {
                            let (third, s) = s.split_at(length_3);

                            if is_component(third) && is_component(s) {
                                result.push(format!("{}.{}.{}.{}", first, second, third, s));
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
// @lc code=end
