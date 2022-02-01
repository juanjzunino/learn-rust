#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0.0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that aren't methods
    fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };

    let rect2 = Rectangle {
        width: 10.0,
        height: 40.0,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Debug fmt
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // dbg macro
    dbg!(&rect1);

    // Rectangle method
    println!(
        "The area of the rectangle is {} squeare pixels",
        rect1.area()
    );

    // Square
    let sq = Rectangle::square(30.);
    println!("{:?}", sq);
}
