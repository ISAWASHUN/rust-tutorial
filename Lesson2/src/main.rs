fn fizzbuzz(end: i32) {
    let mut cnt: i32 = 1;
    while cnt <= end {
        if cnt % 15 == 0 {
            println!("FizzBuzz");
        } else if cnt % 3 == 0 {
            println!("Fizz");
        } else if cnt % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", cnt);
        }
        cnt += 1;
    }
}

fn main() {
    fizzbuzz(30);
}