/*
 * @lc app=leetcode id=1333 lang=rust
 *
 * [1333] Filter Restaurants by Vegan-Friendly, Price and Distance
 */

// @lc code=start
impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
       let mut ans:  Vec<Vec<i32>>= restaurants.iter().cloned().filter(|x| x[3]<=max_price && x[4]<=max_distance  && if vegan_friendly !=0 { x[2]==vegan_friendly}else{true} ).collect();
       ans.sort_by(|a,b| (b[1],b[0]).cmp(&(a[1],a[0])));
       ans.iter().map(|x|x[0]).collect::<Vec<_>>()
    }
}
// @lc code=end

