mod front_of_house; // Declares module front_of_house, who's body is the contents of the file src/front_of_house.rs
// this declares it for the whole project, since this is the root file
// for a binary package, main.rs is the root

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    /*crate::*/front_of_house::hosting::seat_at_table(); // front_of_house can be called here because it is a sibling of eat at restaurant
    // crate:: is an absolute path
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
}

mod back_of_house {
    pub fn cook_order() {}

    fn fix_incoherent_order() {
        cook_order();
        super::front_of_house::serving::serve_order(); // `super::super::` is legal, just like `../../` in a path
        crate::front_of_house::serving::serve_order(); // Either on of these works
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // This cannot be set from outside the back_of_house module, so it cannot be constructed directly
    }
    
    impl Breakfast { // Because Breakfast contains a private felid, I need to provide a constructor, or else it won't be constructable outside of this module
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Potato { // All variants of an enum are public if the enum is public
        french_fry,
        tater_tot,
        mashed,
        baked,
    }
}