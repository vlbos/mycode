/*
 * @lc app=leetcode id=592 lang=rust
 *
 * [592] Fraction Addition and Subtraction
 */

// @lc code=start
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut ans = vec![0, 1];
        let gcd = |l: i32, r: i32| -> i32 {
            let (mut a, mut b) = if l > r { (l, r) } else { (r, l) };
            while b != 0 {
                std::mem::swap(&mut a, &mut b);
                b %= a;
            }
            a
        };
        let lcm = |l: i32, r: i32| -> i32 {
            if l == r {
                return l;
            }
            let (mut a, mut b) = if l > r { (l, r) } else { (r, l) };
            if a % b == 0 {
                return a;
            }
            for i in (a..=a * b).step_by(a as usize) {
                if i % b == 0 {
                    return i;
                }
            }
            a
        };

        let s = expression
            .split('/')
            .map(|x| {
                let signed = if x.find('-').is_some() {-1}else{1};
                let xx = x
                    .split(&['-', '+'][..])
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>();
                if xx.len() == 2 {
                    vec![xx[0].parse::<i32>().unwrap(), signed*xx[1].parse::<i32>().unwrap()]
                } else {
                    vec![x.parse::<i32>().unwrap()]
                }
            })
            .flat_map(|x| x).collect::<Vec<i32>>()
            .chunks(2)
            .fold(ans, |mut s, x| {
                    let numerator1=s[0];
                    let denominator1 =s[1];
                    let numerator2 = *x.first().unwrap();
                    let denominator2 = *x.last().unwrap();
                    let denominator = lcm(denominator1,denominator2);
                    let newnumerator1 = numerator1*(denominator/denominator1);
                    let newnumerator2 = numerator2*(denominator/denominator2);
                    let numerator = newnumerator1+newnumerator2;
                    if numerator==0{
                    return vec![0,1];}
                    let d = gcd(numerator,denominator).abs();
                    vec![numerator/d, denominator/d]
            })
           ;
        format!("{}/{}", s[0],s[1])
    }
}
// @lc code=end
