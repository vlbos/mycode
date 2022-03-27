/*
 * @lc app=leetcode id=2115 lang=rust
 *
 * [2115] Find All Possible Recipes from Given Supplies
 */

// @lc code=start
impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
         use std::collections::HashMap;
        let mut depend = HashMap::new();
        let mut cnt = HashMap::new();
        for (i, ingredient) in ingredients.iter().enumerate() {
            for ing in ingredient {
                depend.entry(ing).or_insert(Vec::new()).push(recipes[i].clone());
            }
            cnt.entry(recipes[i].clone()).or_insert(ingredient.len());
        }
        let mut q = std::collections::VecDeque::new();
        for s in supplies {
            q.push_back(s.clone());
        }
        let mut ans = Vec::new();
        while let Some(ing) = q.pop_front() {
            for r in depend.get(&ing).unwrap_or(&Vec::new()) {
                if let Some(mut n) = cnt.get_mut(r) {
                    *n -= 1;
                    if *n == 0 {
                        ans.push(r.clone());
                        q.push_back(r.clone());
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
