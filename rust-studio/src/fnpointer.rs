// #[derive(Debug, Clone, Default, Eq,Ord, PartialOrd,PartialEq)]
// pub struct AddressValue<F: Fn(&AddressValue<F>) -> i32>{
//     get_balance: Option<F>,
// }

#[derive(Debug, Clone, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct AddressValue {
    get_balance: Option<Box<dyn for<'a> Fn(&'a AddressValue) -> i32>>,
}

fn main() {
    let _av = AddressValue {
        get_balance: Some(Box::new(|_av: &AddressValue| -> i32 { 0 })),
    };
}
