#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // Can be called statically Rectangle::area(self: &Rectangle) or on an instance rect.area()
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        self.width + self.height
    }
    fn can_hold(&self, o: &Rectangle) -> bool {
        self.width > o.width && self.height > o.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {:#?} square units", Rectangle::area(&Rectangle { width: 2, height: 2, }));
}

fn other() {
    impl Rectangle { // Apparently this works anyway
        fn square(s: u32) -> Rectangle {
            Rectangle {
                width: s,
                height: s,
            }
        }
    }
}