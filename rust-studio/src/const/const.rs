struct Item<const I: i32>;
impl Item<0> {
    fn fun_for_0() {}
    fn for_0(self) {}
}

fn main(){
Item::<0>::fun_for_0();
Item::<1>::fun_for_0(); // Error
}