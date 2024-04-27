// enum Shape {
//     Circle,
//     Square(u32),
//     Triangle{base: u32, height: u32},
// }

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
    // let c: Shape = Shape::Circle;
    // let s: Shape = Shape::Square(10);
    // let t: Shape = Shape::Triangle{base: 10, height: 20};

    // match c {
    //     Shape::Circle => println!("This is a circle"),
    //     Shape::Square(side) => println!("This is a square with side {}", side),
    //     Shape::Triangle{base, height} => println!("This is a triangle with base {} and height {}", base, height),
 // }

    // let a: Option<i32> = Some(1);
    // let b: Option<&str> = Some("str");
    // let c: Option<i32> = None;
    
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let val: Option<&i32> = v.get(2);

    match val {
        Some(x) if *x == 1 => println!("Value is {}", x),
        None => println!("No value"),
    }

    // if let Some(x) = val {
    //     println!("Value is {}", x)
    // }
}
