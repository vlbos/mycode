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


impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        let n=*nums.iter().max().unwrap()+1;
        let mut parent:Vec<i32>=(0..n).collect();
        let mut rank=vec![0;n as usize];
        fn find(x:i32,parent:&mut Vec<i32>)->i32{
            let px=parent[x as usize];
            if px!=x{
                parent[x as usize]=find(px,parent);
            }
            parent[x as usize]
        }
        let unite=|x:i32,y:i32,parent:&mut Vec<i32>,rank:&mut Vec<i32>|{
            let (x,y)=(find(x,parent),find(y,parent));
            if x!=y{
                let (x,y)=if rank[x as usize]<rank[y as usize]{(y,x)}else{ if rank[x as usize]==rank[y as usize]{rank[x as usize]+=1;} (x,y)};
                parent[x as usize]=y;
            }
        };
        for &num in &nums{
            let mut i=2;
            while i*i<=num{
                if num%i==0{
                    unite(num,i,&mut parent,&mut rank);
                    unite(num,num/i,&mut parent,&mut rank);
                }
                i+=1;
            }
        }
        *nums.into_iter().fold(std::collections::HashMap::new(),|mut a,num| {*a.entry(find(num,&mut parent)).or_insert(0)+=1;a}).values().max().unwrap()
    }
}