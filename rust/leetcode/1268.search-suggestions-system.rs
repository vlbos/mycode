/*
 * @lc app=leetcode id=1268 lang=rust
 *
 * [1268] Search Suggestions System
 */

// @lc code=start
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let mut query = String::new();
        let mut ans = Vec::new();
        for c in search_word.chars() {
            query.push(c);
            if let Ok(i) | Err(i) = products.binary_search(&query) {
                let mut selects = Vec::new();
                for j in 0..3 {
                    if i + j < products.len() && products[i + j].starts_with(&query) {
                        selects.push(products[i + j].clone());
                    }
                }
                ans.push(selects);
            }
        }
        ans
    }
}
// @lc code=end
