// # [2548. Maximum Price to Fill a Bag](https://leetcode.com/problems/maximum-price-to-fill-a-bag)
// ## Description

//  You are given a 2D integer array  items  where  items[i] = [price i , weight i ]  denotes the price and weight of the  i th   item, respectively.

//  You are also given a  positive  integer  capacity .

//  Each item can be divided into two items with ratios  part1  and  part2 , where  part1 + part2 == 1 .

// 	 The weight of the first item is  weight i  * part1  and the price of the first item is  price i  * part1 .
// 	 Similarly, the weight of the second item is  weight i  * part2  and the price of the second item is  price i  * part2 .

//  Return   the maximum total price  to fill a bag of capacity   capacity   with given items . If it is impossible to fill a bag return  -1 . Answers within  10 -5   of the  actual answer  will be considered accepted.

//  Example 1:

//  Input:  items = [[50,1],[10,8]], capacity = 5
//  Output:  55.00000
//  Explanation:
// We divide the 2 nd  item into two parts with part1 = 0.5 and part2 = 0.5.
// The price and weight of the 1 st  item are 5, 4. And similarly, the price and the weight of the 2 nd  item are 5, 4.
// The array items after operation becomes [[50,1],[5,4],[5,4]].
// To fill a bag with capacity 5 we take the 1 st  element with a price of 50 and the 2 nd  element with a price of 5.
// It can be proved that 55.0 is the maximum total price that we can achieve.

//  Example 2:

//  Input:  items = [[100,30]], capacity = 50
//  Output:  -1.00000
//  Explanation:  It is impossible to fill a bag with the given item.

//   Constraints:

// 	  1 <= items.length <= 10^5
// 	  items[i].length == 2
// 	  1 <= price i , weight i  <= 10^4
// 	  1 <= capacity <= 10^9
//   double max_price(vector<vector<int>>& items, int capacity) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_price(items: Vec<Vec<i32>>, capacity: i32) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_feq;
    #[test]
    pub fn test_max_price_1() {
        assert_feq!(
            55.00000,
            Solution::max_price(vec![vec![50, 1], vec![10, 8]], 5),
            1e-5
        );
    }
    #[test]
    pub fn test_max_price_2() {
        assert_feq!(-1.00000, Solution::max_price(vec![vec![100, 30]], 50), 1e-5);
    }
}
