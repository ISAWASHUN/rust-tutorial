enum Shape {
    Circle,
    Square(u32),
    Triangle{base: u32, height: u32},
}

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn new(width: u32, height: u32) -> Rectangle {
//         Rectangle { width, height }
//     }
// }

fn main() {
    let c: Shape = Shape::Circle;
    let s: Shape = Shape::Square(10);
    let t: Shape = Shape::Triangle{base: 10, height: 20};

    match c {
        Shape::Circle => println!("This is a circle"),
        Shape::Square(side) => println!("This is a square with side {}", side),
        Shape::Triangle{base, height} => println!("This is a triangle with base {} and height {}", base, height),
    }
}