mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}


use crate::customer::eat_at_restaurant;
fn main() {
    eat_at_restaurant();
}
