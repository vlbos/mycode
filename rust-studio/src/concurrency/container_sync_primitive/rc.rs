use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::rc::Rc;
pub fn main() {
    use std::rc::Rc;
    let data = Rc::new(42);
    let _reference1 = Rc::clone(&data);
    let _reference2 = Rc::clone(&data);
    // data 3
    // reference1 reference2
    rc_refcell_example();
}
pub fn rc_refcell_example() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }
    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
}
