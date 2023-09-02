/*
 * @lc app=leetcode id=399 lang=rust
 *
 * [399] Evaluate Division
 */

// @lc code=start
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut m = std::collections::HashMap::new();
        let mut id = 0;
        for e in &equations {
            for i in 0..2 {
                if !m.contains_key(&e[i]) {
                    m.insert(e[i].clone(), id);
                    id += 1;
                }
            }
        }
        let mut parent = vec![0; id];
        let mut weight = vec![1.0; id];
        for i in 0..parent.len() {
            parent[i] = i ;
        }
        fn find(parent: &mut Vec<usize>, weight: &mut Vec<f64>, x: usize)->usize {
            if parent[x] != x {
                let origin = parent[x];
                parent[x] = find(parent, weight, origin);
                weight[x] *= weight[origin];
            }
            parent[x]
        }
        fn union(parent: &mut Vec<usize>, weight: &mut Vec<f64>, x: usize, y: usize, val: f64) {
            let px = find(parent, weight, x);
            let py = find(parent, weight, y);
            if px != py {
                parent[px] = py;
                weight[px] = val * weight[y] / weight[x];
            }
        }
        for (i, e) in equations.iter().enumerate() {
            match (m.get(&e[0]), m.get(&e[1])) {
                (Some(x), Some(y)) => union(&mut parent, &mut weight, *x, *y, values[i]),
                _ => (),
            }
        }
        let mut ans = vec![-1.0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            match (m.get(&q[0]), m.get(&q[1])) {
                (Some(x), Some(y)) => {
                    let px = find(&mut parent,&mut  weight, *x);
                    let py = find(&mut parent, &mut weight, *y);
                    if px == py {
                        ans[i] = weight[*x] / weight[*y];
                    }
                }
                _ => (),
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut id=std::collections::HashMap::new();
        for equation in &equations{
            for e in equation{
                let i=id.len();
                id.entry(e.clone()).or_insert(i);
            }
        }
        let n=id.len();
        let mut f:Vec<usize>=(0..n).collect();
        let mut w=vec![1.0;n];
        fn find(x:usize,f:&mut Vec<usize>,w:&mut Vec<f64>)->usize{
            let px=f[x];
            if px!=x{
                let fx=find(px,f,w);
                w[x]*=w[f[x]];
                f[x]=fx;
            }
            f[x]
        }
        let unite=|x:usize,y:usize,val:f64,f:&mut Vec<usize>,w:&mut Vec<f64>|{
            let (px,py)=(find(x,f,w),find(y,f,w));
            if px!=py{
                f[px]=py;
                w[px]=val*w[y]/w[x];
            }
        };
        for (e,&v) in equations.iter().zip(&values){
            unite(id[&e[0] ],id[&e[1]],v,&mut f,&mut w);
        }
        let mut ans=vec![-1.0;queries.len()];
        for (i,q) in queries.iter().enumerate(){
            if let (Some(&x),Some(&y))=(id.get(&q[0]),id.get(&q[1])){
                if find(x,&mut f,&mut w)==find(y,&mut f,&mut w){
                    ans[i]=w[x]/w[y];
                }
            }
        }
        ans
    }
}