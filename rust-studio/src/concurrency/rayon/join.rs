use rayon::join;
pub fn main1() {
    let result = join(|| expensive_operation1(), || expensive_operation2());
    let final_result = result.0 + result.1;
    println!("Final Result: {}", final_result);
}
fn expensive_operation1() -> i32 {
    42
}
fn expensive_operation2() -> i32 {
    58
}
pub fn main() {
    main1();
    let mut v = vec![5, 1, 8, 22, 0, 44];
    quick_sort(&mut v);
    assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);
    fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
        if v.len() > 1 {
            let mid = partition(v);
            let (lo, hi) = v.split_at_mut(mid);
            rayon::join(|| quick_sort(lo), || quick_sort(hi));
        }
    }
}
// Partition`<=`
// " "
fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);

            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
