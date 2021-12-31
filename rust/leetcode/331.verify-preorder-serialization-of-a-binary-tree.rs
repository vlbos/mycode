/*
 * @lc app=leetcode id=331 lang=rust
 *
 * [331] Verify Preorder Serialization of a Binary Tree
 */

// @lc code=start
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut s: Vec<&str> = Vec::new();
        let p = preorder.split(',').collect::<Vec<&str>>();
        let mut slots = 1;
        for (i, o) in p.iter().enumerate() {
            if slots == 0 {
                return false;
            }
            if *o == "#" {
                slots -= 1;
            } else {
                slots += 1;
            }
        }
        slots == 0
    }
}
// @lc code=end
