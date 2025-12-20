// ## [3549\. Multiply Two Polynomials 🔒](https://leetcode.com/problems/multiply-two-polynomials)

// ## Description

// You are given two integer arrays `poly1` and `poly2`,
// where the element at index `i` in each array represents the coefficient of `xi` in a polynomial.

// Let `A(x)` and `B(x)` be the polynomials represented by `poly1` and `poly2`, respectively.

// Return an integer array `result` representing the coefficients of the product polynomial `R(x) = A(x) * B(x)`,
//  where `result[i]` denotes the coefficient of `xi` in `R(x)`.

// **Example 1:**

// **Input:** poly1 = \[3,2,5\], poly2 = \[1,4\]

// **Output:** \[3,14,13,20\]

// **Explanation:**

// +   `A(x) = 3 + 2x + 5x2` and `B(x) = 1 + 4x`
// +   `R(x) = (3 + 2x + 5x2) * (1 + 4x)`
// +   `R(x) = 3 * 1 + (3 * 4 + 2 * 1)x + (2 * 4 + 5 * 1)x2 + (5 * 4)x3`
// +   `R(x) = 3 + 14x + 13x2 + 20x3`
// +   Thus, result = `[3, 14, 13, 20]`.

// **Example 2:**

// **Input:** poly1 = \[1,0,-2\], poly2 = \[-1\]

// **Output:** \[-1,0,2\]

// **Explanation:**

// +   `A(x) = 1 + 0x - 2x2` and `B(x) = -1`
// +   `R(x) = (1 + 0x - 2x2) * (-1)`
// +   `R(x) = -1 + 0x + 2x2`
// +   Thus, result = `[-1, 0, 2]`.

// **Example 3:**

// **Input:** poly1 = \[1,5,-3\], poly2 = \[-4,2,0\]

// **Output:** \[-4,-18,22,-6,0\]

// **Explanation:**

// +   `A(x) = 1 + 5x - 3x2` and `B(x) = -4 + 2x + 0x2`
// +   `R(x) = (1 + 5x - 3x2) * (-4 + 2x + 0x2)`
// +   `R(x) = 1 * -4 + (1 * 2 + 5 * -4)x + (5 * 2 + -3 * -4)x2 + (-3 * 2)x3 + 0x4`
// +   `R(x) = -4 -18x + 22x2 -6x3 + 0x4`
// +   Thus, result = `[-4, -18, 22, -6, 0]`.

// **Constraints:**

// +   `1 <= poly1.length, poly2.length <= 5 * 104`
// +   `-103 <= poly1[i], poly2[i] <= 103`
// +   `poly1` and `poly2` contain at least one non-zero coefficient.

// // vector<long long> multiply(vector<int>& poly1, vector<int>& poly2) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn multiply(poly1: Vec<i32>, poly2: Vec<i32>) -> Vec<i64> {
        #[derive(Default, Clone, Copy)]
        struct Complex {
            pub re: f64,
            pub im: f64,
        }
        impl Complex {
            pub fn new(re: f64, im: f64) -> Self {
                Self { re, im }
            }
        }
        use std::ops::{Add, Div, Mul, Sub};
        impl Div<f64> for Complex {
            type Output = Self;
            fn div(self, rhs: f64) -> Self::Output {
                Self::new(self.re / rhs, self.im / rhs)
            }
        }
        impl Mul for Complex {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(
                    self.re * rhs.re - self.im * rhs.im,
                    self.re * rhs.im + self.im * rhs.re,
                )
            }
        }
        impl Add for Complex {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.re + rhs.re, self.im + rhs.im)
            }
        }
        impl Sub for Complex {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.re - rhs.re, self.im - rhs.im)
            }
        }
        let (n1, n2) = (poly1.len(), poly2.len());
        let n = n1 + n2 - 1;
        let sz = n.next_power_of_two();
        let (mut a, mut b) = (vec![Complex::default(); sz], vec![Complex::default(); sz]);
        a.iter_mut().zip(&poly1).for_each(|(v, &x)| {
            v.re = x as f64;
        });
        b.iter_mut().zip(&poly2).for_each(|(v, &x)| {
            v.re = x as f64;
        });
        use std::f64::consts::PI;
        fn fft(a: &mut Vec<Complex>, inverse: bool) {
            let n = a.len();
            let mut j = 0;
            for i in 1..n {
                let mut bit = n >> 1;
                while j & bit != 0 {
                    j ^= bit;
                    bit >>= 1;
                }
                j ^= bit;
                if i < j {
                    a.swap(i, j);
                }
            }
            let mut len = 2;
            while len <= n {
                let angle = 2.0 * PI / len as f64 * (if inverse { -1.0 } else { 1.0 });
                let w_len = Complex::new(angle.cos(), angle.sin());
                for i in (0..n).step_by(len) {
                    let mut w = Complex::new(1.0, 0.0);
                    for j in 0..len / 2 {
                        let (u, v) = (a[i + j], a[i + j + len / 2] * w);
                        a[i + j] = u + v;
                        a[i + j + len / 2] = u - v;
                        w = w * w_len;
                    }
                }
                len <<= 1;
            }
            if inverse {
                a.iter_mut().for_each(|v| {
                    *v = *v / n as f64;
                });
            }
        }
        fft(&mut a, false);
        fft(&mut b, false);
        a.iter_mut().zip(&b).for_each(|(x, &y)| {
            *x = *x * y;
        });
        fft(&mut a, true);
        a[..n].iter().map(|v| v.re.round() as i64).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_multiply_1() {
        assert_eq!(
            vec![3, 14, 13, 20],
            Solution::multiply(vec![3, 2, 5], vec![1, 4])
        );
    }
    #[test]
    pub fn test_multiply_2() {
        assert_eq!(vec![-1, 0, 2], Solution::multiply(vec![1, 0, -2], vec![-1]));
    }
    #[test]
    pub fn test_multiply_3() {
        assert_eq!(
            vec![-4, -18, 22, -6, 0],
            Solution::multiply(vec![1, 5, -3], vec![-4, 2, 0])
        );
    }
}
