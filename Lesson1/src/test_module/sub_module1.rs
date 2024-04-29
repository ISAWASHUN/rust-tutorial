pub fn test1_fn1() {
  println!("test1_fn1");
}

fn test2_fn2() {
  println!("test2_fn1");
}

pub struct TestStruct {
  pub field1: i32,
  field2: i32,
}

impl TestStruct {
  pub fn new() -> TestStruct {
    TestStruct {
      field1: 0,
      field2: 0,
    }
  }
}
