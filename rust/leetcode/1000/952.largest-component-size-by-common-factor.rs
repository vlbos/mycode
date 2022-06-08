/*
 * @lc app=leetcode id=952 lang=rust
 *
 * [952] Largest Component Size by Common Factor
 */

// @lc code=start
impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut factored = Vec::new();
        for &x in &nums {
            let (mut d, mut x) = (2, x);
            let mut fact = Vec::new();
            while d * d <= x {
                if x % d == 0 {
                    while x % d == 0 {
                        x /= d;
                    }
                    fact.push(d);
                }
                d += 1;
            }
            if x > 1 || fact.is_empty() {
                fact.push(x);
            }
            factored.push(fact);
        }
        let mut primes: Vec<i32> = factored.iter().flatten().cloned().collect();
        primes.sort();
        primes.dedup();
        let primes_to_index: std::collections::HashMap<usize, usize> = primes
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v as usize, i))
            .collect();
        let mut parent: Vec<usize> = (0..primes.len()).collect();
        fn find(x: usize, parent: &mut Vec<usize>) -> usize {
            if parent[x] == x {
                return x;
            }
            parent[x] = find(parent[x],parent);
            parent[x]
        }
        fn merge(x: usize, y: usize, parent: &mut Vec<usize>){
            let (px, py) = (find(x, parent), find(y, parent));
             parent[px] = py;
        }
        for fact in &factored {
            for &x in fact {
                merge(
                    *primes_to_index.get(&(fact[0] as usize)).unwrap(),
                    *primes_to_index.get(&(x as usize)).unwrap(), &mut parent
                );
            }
        }
        let mut count = vec![0; primes.len()];
        for fact in &factored {
                count[find(*primes_to_index.get(&(fact[0] as usize)).unwrap(),&mut parent)] += 1;
        }
        *count.iter().max().unwrap()
    }
}
// @lc code=end
