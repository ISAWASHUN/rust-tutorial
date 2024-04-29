mod test_module {
    pub mod sub_module1 {
        pub fn test1_fn1() {
            println!("test1_fn1");
        }

        fn test2_fn2() {
            println!("test2_fn1");
        }
    }

    mod sub_module2 {
        pub fn test1_fn1() {
            println!("test1_fn2");
        }

        fn test2_fn2() {
            println!("test2_fn2");
        }
    }
}

use test_module::sub_module1;

fn main() {
    crate::test_module::sub_module1::test1_fn1();
    self::test_module::sub_module1::test1_fn1();
    test_module::sub_module1::test1_fn1();
    test_module::sub_module1::test1_fn1();
    // test_module::sub_module2::test1_fn1();
    // test_module::sub_module1::test2_fn2(); // error: function `test2_fn2` is private
}
