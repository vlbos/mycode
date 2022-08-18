// 1257\. Smallest Common Region
// =============================

// You are given some lists of `regions` where the first region of each list includes all other regions in that list.

// Naturally, if a region `X` contains another region `Y` then `X` is bigger than `Y`. Also by definition a region X contains itself.

// Given two regions `region1`, `region2`, find out the **smallest** region that contains both of them.

// If you are given regions `r1`, `r2` and `r3` such that `r1` includes `r3`, it is guaranteed there is no `r2` such that `r2` includes `r3`.

// It's guaranteed the smallest region exists.

// **Example 1:**

// **Input:** regions = \[\["Earth","North America","South America"\],
// \["North America","United States","Canada"\],
// \["United States","New York","Boston"\],
// \["Canada","Ontario","Quebec"\],
// \["South America","Brazil"\]\],
// region1 = "Quebec",
// region2 = "New York"
// **Output:** "North America"

// **Constraints:**

// *   `2 <= regions.length <= 10^4`
// *   `region1 != region2`
// *   All strings consist of English letters and spaces with at most 20 letters.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn find_smallest_region(
        regions: Vec<Vec<String>>,
        region1: String,
        region2: String,
    ) -> String {
        use std::collections::{HashMap, HashSet};
        let mut c2p = HashMap::new();
        for r in &regions {
            for c in &r[1..] {
                c2p.insert(c.clone(), r[0].clone());
            }
        }
        let mut p1 = HashSet::new();
        let mut r1 = region1.clone();
        while let Some(r) = c2p.get(&r1) {
            p1.insert(r1.clone());
            if r == &r1 {
                break;
            }
            r1 = r.clone();
        }
        let mut r2 = region2.clone();
        while let Some(r) = c2p.get(&r2) {
            if p1.contains(r) {
                return r.clone();
            }
            if r == &r2 {
                break;
            }
            r2 = r.clone();
        }
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_smallest_region_1() {
        assert_eq!(
            String::from("North America"),
            Solution::find_smallest_region(
                vec![
                    ["Earth", "North America", "South America"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<String>>(),
                    ["North America", "United States", "Canada"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<String>>(),
                    ["United States", "New York", "Boston"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<String>>(),
                    ["Canada", "Ontario", "Quebec"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<String>>(),
                    ["South America", "Brazil"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                ],
                String::from("Quebec"),
                String::from("New York")
            )
        );
    }
}
