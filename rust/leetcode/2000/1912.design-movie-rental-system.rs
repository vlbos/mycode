/*
 * @lc app=leetcode id=1912 lang=rust
 *
 * [1912] Design Movie Rental System
 */

// @lc code=start
use std::collections::BTreeSet;
use std::collections::HashMap;
struct MovieRentingSystem {
    prices: HashMap<Vec<i32>, i32>,
    valid: HashMap<i32, BTreeSet<Vec<i32>>>,
    rent: BTreeSet<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut prices = HashMap::new();
        let mut valid = HashMap::new();
        for entry in &entries {
            prices.insert(entry[..2].to_vec(), entry[2]);
            valid
                .entry(entry[1])
                .or_insert(BTreeSet::new())
                .insert(vec![entry[2], entry[0]]);
        }
        Self {
            prices,
            valid,
            rent: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        if let Some(v) = self.valid.get(&movie) {
            let mut ans = Vec::new();
            let mut i = 0;
            for vv in v {
                ans.push(vv[1]);
                if i == 4 {
                    break;
                }
                i += 1;
            }
            ans
        } else {
            Vec::new()
        }
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        if let Some(&price) = self.prices.get(&vec![shop, movie]) {
            self.valid.entry(movie).and_modify(|x| {
                x.remove(&vec![price, shop]);
            });
            self.rent.insert(vec![price, shop, movie]);
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        if let Some(&price) = self.prices.get(&vec![shop, movie]) {
            self.valid
                .entry(movie)
                .or_insert(BTreeSet::new())
                .insert(vec![price, shop]);
            self.rent.remove(&vec![price, shop, movie]);
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut i = 0;
        for v in &self.rent {
            ans.push(v[1..].to_vec());
            if i == 4 {
                break;
            }
            i += 1;
        }
        ans
    }
}


/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
// @lc code=end
