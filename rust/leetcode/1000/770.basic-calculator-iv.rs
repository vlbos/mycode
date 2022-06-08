/*
 * @lc app=leetcode id=770 lang=rust
 *
 * [770] Basic Calculator IV
 */

// @lc code=start
use std::collections::HashMap;
#[derive(Debug)]
struct Poly {
    count: HashMap<Vec<String>, i32>,
}
impl Poly {
    fn new() -> Self {
        Self {
            count: HashMap::new(),
        }
    }
    fn update(&mut self, key: &Vec<String>, val: i32) {
        *self.count.entry(key.clone()).or_insert(0) += val;
    }

    fn add(&self, other: &Poly) -> Poly {
        let mut ans = Poly {
            count: HashMap::new(),
        };
        for (k, &v) in &self.count {
            ans.update(k, v);
        }
        for (k, &v) in &other.count {
            ans.update(k, v);
        }
        ans
    }
    fn sub(&self, other: &Poly) -> Poly {
        let mut ans = Poly {
            count: HashMap::new(),
        };
        for (k, &v) in &self.count {
            ans.update(k, v);
        }
        for (k, &v) in &other.count {
            ans.update(k, -v);
        }
        ans
    }
    fn mul(&self, other: &Poly) -> Poly {
        let mut ans = Poly {
            count: HashMap::new(),
        };
        for (k1, &v1) in &self.count {
            for (k2, &v2) in &other.count {
                let mut new_k = Vec::new();
                new_k.extend(k1.clone());
                new_k.extend(k2.clone());
                new_k.sort();
                ans.update(&new_k, v1 * v2);
            }
        }
        ans
    }
    fn evaluate(&self, eval_map: &HashMap<String, i32>) -> Poly {
        let mut ans = Poly {
            count: HashMap::new(),
        };
        for (k, &v) in &self.count {
            let mut c = v;
            let mut free = Vec::new();
            for token in k {
                if let Some(n) = eval_map.get(token) {
                    c *= n;
                } else {
                    free.push(token.clone());
                }
            }
            ans.update(&free, c);
        }
        ans
    }

    fn to_list(&self) -> Vec<String> {
        let mut ans = Vec::new();
        let mut keys: Vec<Vec<String>> = self.count.keys().cloned().collect();
        keys.sort_by(|a, b| {
            if a.len() != b.len() {
                b.len().cmp(&a.len())
            } else {
                a.cmp(&b)
            }
        });
         println!("to={:?},{:?}",keys,self.count);
        for key in &keys {
            let v = *self.count.get(key).unwrap_or(&0);
            if v == 0 {
                continue;
            }
            let mut word = v.to_string()+if key.is_empty() {""}else {"*"};
            word.push_str(key.join("*").as_str());
            ans.push(word.clone());
        }
        ans
    }
}

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        fn combine (left: &Poly, right: &Poly, symbol: char) -> Poly {
            match symbol {
                '+' => left.add(right),
                '-' => left.sub(right),
                '*' => left.mul(right),
                _ => Poly {
                count: HashMap::new(),
            },
            }
        }
        fn make (expr: &String)-> Poly {
            let mut ans = Poly {
                count: HashMap::new(),
            };
            if let Ok(n) = expr.parse::<i32>() {
                ans.update(&Vec::new(), n);
            } else {
                ans.update(&vec![expr.clone()],1);
            }
            ans
        }
        fn parse(expr: &String) -> Poly {
            let mut ans = Poly {
                count: HashMap::new(),
            };
            let mut bucket = Vec::new();
            let mut symbols = Vec::new();
            let mut i = 0;
            let be = expr.as_bytes();
            let n = be.len();
            while i < n {
                if be[i] == b'(' {
                    let mut bal = 0;
                    let mut j = i;
                    while j<n {
                        if be[j] == b'(' {
                            bal += 1;
                        }
                        if be[j] == b')' {
                            bal -= 1;
                        }
                        if bal == 0 {
                            break;
                        }
                        j+=1;
                    }
                    if j>i+1 {bucket.push(parse(&expr[i + 1..j].to_string()));}

                    i = j;
                } else if (be[i] as char).is_ascii_alphanumeric() {
                    let mut j =i;
                    let mut flag = true;
                    while j<n{
                        if be[j] == b' ' {
                            bucket.push(make(&expr[i..j].to_string()));
                            flag=false;
                            break ;
                        }
                        j+=1;
                    }
                    if flag{
                    bucket.push(make(&expr[i..].to_string()));
                    }
                    i = j;
                } else if be[i] != b' ' {
                    symbols.push(be[i]);
                }
                i += 1;
            }

            let sn = symbols.len();
            for j in (0..sn).rev() {
                if symbols[j] == b'*' {
                    bucket[j] = combine(&bucket[j], &bucket[j + 1], symbols[j] as char);
                    bucket.remove(j + 1);
                    symbols.remove(j);
                }
            }
            if bucket.is_empty() {
                return Poly {
                    count: HashMap::new(),
                };
            }
            if symbols.is_empty() {
               return  bucket.remove(0);
            }
            let mut ans = Poly {
                    count: HashMap::new(),
                };
            
            for j in 0..symbols.len() {
                ans = combine(if j==0 {&bucket[0]}else{&ans}, &bucket[j + 1], symbols[j] as char);
            }
            ans
        }
        let eval_map: HashMap<String, i32> = evalvars.iter().cloned().zip(evalints).collect();
        parse(&expression).evaluate(&eval_map).to_list()
    }
}
// @lc code=end
