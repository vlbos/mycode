use cuckoofilter::CuckooFilter;
fn main() {
    let value: &str = "hello world";
    // Create cuckoo filter with default max capacity of 1000000 items
    let mut cf = CuckooFilter::new();
    // Add data to the filter
    cf.add(value).unwrap();
    // Lookup if data is in the filter
    let success = cf.contains(value);
    assert!(success);
    // Test and add to the filter (if data does not exists then add)
    let success = cf.test_and_add(value).unwrap();
    assert!(!success);
    // Remove data from the filter.
    let success = cf.delete(value);
    assert!(success);
}
