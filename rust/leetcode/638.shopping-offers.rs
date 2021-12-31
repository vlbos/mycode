/*
 * @lc app=leetcode id=638 lang=rust
 *
 * [638] Shopping Offers
 */

// @lc code=start
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();
        let mut fs = Vec::new();
        for s in &special {
            let mut t = 0;
            let mut tc = 0;
            for i in 0..n {
                t += s[i] * price[i];
                tc += s[i];
            }
            if tc > 0 && t > s[n] {
                fs.push(s.clone());
            }
        }

        fn dfs(price: &Vec<i32>,
            special: &Vec<Vec<i32>>,
            needs: &Vec<i32>,
            m: &mut std::collections::HashMap<Vec<i32>, i32>,
        )->i32 {
            if !m.contains_key(needs) {
                let n = needs.len();
                let mut mp = 0;
                for i in 0..needs.len() {
                    mp += needs[i] * price[i];
                }
                for s in special {
                    let t = s[n];
                    if t > mp {
                        continue;
                    }
                    let mut nn = Vec::new();
                    for i in 0..n {
                        if needs[i] < s[i] {
                            break;
                        }
                        nn.push(needs[i] - s[i]);
                    }
                    if nn.len() == n {
                        mp = mp.min(dfs(price,special, &nn, m) + t);
                    }
                }

                m.insert(needs.clone(),mp);
            }
            m[needs]
        }
        let mut m = std::collections::HashMap::new();
        dfs(&price,&fs, &needs, &mut m)
    }
}
// @lc code=end
