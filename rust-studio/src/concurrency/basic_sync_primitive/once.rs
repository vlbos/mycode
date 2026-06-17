use std::sync::{ONCE_INIT, Once};
static INIT: Once = Once::new();
fn main1() {
    // call_once
    INIT.call_once(|| {
        println!("Initialization code executed!");
    });
    // call_once
    INIT.call_once(|| {
        println!("This won't be printed.");
    });
}

// use std::sync::{ONCE_INIT, Once};
static mut GLOBAL_CONFIG: Option<String> = None;
static GLOBAL_CONFIG_INIT: Once = ONCE_INIT;
fn init_global_config() {
    unsafe {
        GLOBAL_CONFIG = Some("Initialized global configuration".to_string());
    }
}
fn get_global_config() -> &'static str {
    GLOBAL_CONFIG_INIT.call_once(|| init_global_config());
    unsafe {
        // GLOBAL_CONFIG.as_ref().unwrap()
        let ptr = std::ptr::addr_of!(GLOBAL_CONFIG); // 获取原始指针
        (*ptr).as_ref().unwrap()
    }
}
fn main2() {
    println!("{}", get_global_config());
    println!("{}", get_global_config());
}

fn main() {
    main1();
    main2();
    use once_cell::sync::OnceCell;
    static CELL: OnceCell<String> = OnceCell::new();
    assert!(CELL.get().is_none());

    std::thread::spawn(|| {
        let value: &String = CELL.get_or_init(|| "Hello, World!".to_string());
        assert_eq!(value, "Hello, World!");
    })
    .join()
    .unwrap();
    let value: Option<&String> = CELL.get();
    assert!(value.is_some());
    assert_eq!(value.unwrap().as_str(), "Hello, World!");
}
