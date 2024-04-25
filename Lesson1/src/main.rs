//関数
// fn say_hello() {
//     println!("Hello!");
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

fn main() {
    //出力
    // println!("Hello, world!");
    // print!("Hello,");
    // print!("Rust!");

    //変数
    // let a: i32 = 1;
    // println!("{}", a);

    // let mut b: i32 = 2;
    // println!("{}", b);

    //定数
    // const A: i32 = 1;

    //数値型
    // let a = 1;
    // let b: f64 = 2.0;
    // let c: u16 = 3;
    // let d: f32 = 4.0f32;

    //演算は同じ型同士でなければならない

    //型変換
    // let f: f64 = 1 as f64 + 2.0;

    //論理型
    //true or false
    // let g: bool = false;

    //tuple
    // let t1: (i32, bool, f64) = (1, true, 2.0);
    
    // let i: i32 = t1.0;
    // println!("{}", i);

    //配列
    // let l1: [i32; 3] = [1,2,3];

    // println!("{:?}", l1);

    // let l3: &[i32] = &l1[0..=2];
    // println!("{:?}", l3);

    //ベクタ型
    // let v1: Vec<i32> = vec![1,2,3];

    // let mut v3: Vec<i32> = Vec::new();
    // v3.push(1);
    // v3.push(2);
    // v3.push(3);
    // println!("{:?}", v3);

    // let x: Option<i32> = v3.pop();
    // println!("{:?}", x);
    // println!("{:?}", v3);

    //文字型と文字列型
    // let c1: char = 'a';

    // //文字列
    // let s1: &str = "Rust";

    // let s2: String = String::from("Python");
    // let s3: String = "Java".to_string();

    // let mut s4: String = String::from("Hello");
    // s4.push_str(" .Rust");
    // println!("{}", s4);

    // println!("{}", s4 + ". golang");

    // say_hello();
    // println!("{}", add(1,2));

    // 条件分岐
    // let x: i32 = 5;

    // if x > 0 {
    //     println!("OK!");
    // }

    // if x > 0 && x < 10 {
    //     println!("0 < x < 10");
    // }

    // //match
    // let x: i32 = 0;
    // match x {
    //     0 => println!("Zero"),
    //     1 => println!("one"),
    //     _ => println!("Other"),
    // };

    // //loop
    // // let mut cnt: i32 = 0;
    // // loop {
    // //     println!("Hello");
    // //     if cnt == 10 {
    // //         break;
    // //     }
    // //     cnt += 1;
    // // }

    // //while
    // let mut cnt: i32 = 0;
    // while cnt M= 10 {
    //     println!("Hello");
    //     cnt += 1;
    // }

    // //for
    // for i in [1,2,3] {
    //     println!("Hello, {}", i);
    // }


    // let r: Range<i32> = 1..10;
    // for x: i32 in r {
    //     println!("{}", x * x);
    // }

    let mut v1: Vec<i32> = vec![1,2,3];
    println!("{:?}", v1.as_ptr());
    println!("{:?}", &v1[0]);

    println("vl1 length: {}", v1.len());
    println("vl1 capacity: {}", v1.capacity());

    println!("{:?}", v1.as_ptr());
    let v2: Vec<i32> = v1.clone();
    println!("{:?}", v2.as_ptr());

    let = s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let s: String = concat(a: &s1, b: &s2);

    println!("{:?}", s);
    println!("{:?}", s1);
    println!("{:?}", s2);
}

fn concat(a: &String, b: &String) -> String {
    let c: String = format!("{}, {}", a, b);
    c
}

