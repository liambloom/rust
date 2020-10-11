#![allow(dead_code)]
#![allow(unused_variables)]

enum IpAddr {
    V4 {

    },
    V6(String),
}

impl IpAddr {

}

fn main() {
    let localhost: (i32, i32) = (2, 4);
    let x: Option<i32> = Some(2); // This is the closest thing rust has to a null
    println!("{:?}", x);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Massachusetts) => {
                println!("Mass!");
                25
            },
            Coin::Quarter(_other_state) => 25
        }
    }
}
enum UsState {
    Massachusetts,
    California,
    Other,
}