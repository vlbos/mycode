// # [2247. Maximum Cost of Trip With K Highways](https://leetcode.com/problems/maximum-cost-of-trip-with-k-highways)

// ## Description

// A series of highways connect n cities numbered from 0 to n - 1. You are given a 2D integer array highways where highways[i] = [city1i, city2i, tolli] indicates that there is a highway that connects city1i and city2i, allowing a car to go from city1i to city2i and vice versa for a cost of tolli.

// You are also given an integer k. You are going on a trip that crosses exactly k highways. You may start at any city, but you may only visit each city at most once during your trip.

// Return the maximum cost of your trip. If there is no trip that meets the requirements, return -1.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2247.Maximum%20Cost%20of%20Trip%20With%20K%20Highways/images/image-20220418173304-1.png" style="height: 200px; width: 327px;" />
//
// Input: n = 5, highways = [[0,1,4],[2,1,3],[1,4,11],[3,2,3],[3,4,2]], k = 3
// Output: 17
// Explanation:
// One possible trip is to go from 0 -&gt; 1 -&gt; 4 -&gt; 3. The cost of this trip is 4 + 11 + 2 = 17.
// Another possible trip is to go from 4 -&gt; 1 -&gt; 2 -&gt; 3. The cost of this trip is 11 + 3 + 3 = 17.
// It can be proven that 17 is the maximum possible cost of any valid trip.

// Note that the trip 4 -&gt; 1 -&gt; 0 -&gt; 1 is not allowed because you visit the city 1 twice.

//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2247.Maximum%20Cost%20of%20Trip%20With%20K%20Highways/images/image-20220418173342-2.png" style="height: 200px; width: 217px;" />
//
// Input: n = 4, highways = [[0,1,3],[2,3,2]], k = 2
// Output: -1
// Explanation: There are no valid trips of length 2, so return -1.
//

// Constraints:

//
// 	2 <= n <= 15
// 	1 <= highways.length <= 50
// 	highways[i].length == 3
// 	0 <= city1i, city2i <= n - 1
// 	city1i != city2i
// 	0 <= tolli <= 100
// 	1 <= k <= 50
// 	There are no duplicate highways.
//

// int maximumCost(int n, vector<vector<int>>& highways, int k) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
