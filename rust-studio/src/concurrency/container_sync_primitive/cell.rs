use std::borrow::Borrow;
use std::cell::OnceCell;
use std::ops::Deref;
pub fn main1() {
    use std::cell::Cell;
    let x = Cell::new(42);
    let y = &x;
    x.set(10);
    println!("y: {:?}", y.get()); // y: 10

    use std::cell::RefCell;
    let x = RefCell::new(42);
    {
        let y = x.borrow();
        println!("y: {:?}", *y.borrow());
    }
    {
        let mut z = x.borrow_mut();
        *z = 10;
    }
    println!("x: {:?}", x.borrow().deref());
}

pub fn once_cell_example() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none()); // true
    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some()); //true
}

use std::cell::LazyCell;
pub fn main() {
    let lazy: LazyCell<i32> = LazyCell::new(|| {
        println!("initializing");
        0
    });
    println!("ready");
    println!("{}", *lazy); // 46
    println!("{}", *lazy); // 46

    use std::collections::HashMap;
    use std::sync::LazyLock;

    static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
        println!("initializing");
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        m
    });

    main1();

    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    })
    .join()
    .unwrap();
    println!("{:?}", HASHMAP.get(&74));
}
