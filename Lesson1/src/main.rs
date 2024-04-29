mod test_module;

use test_module::{sub_module1, sub_module2};

fn main() {
    crate::test_module::sub_module1::test1_fn1();
    self::test_module::sub_module1::test1_fn1();
    test_module::sub_module1::test1_fn1();
    test_module::sub_module1::test1_fn1();
    // test_module::sub_module2::test1_fn1();
    // test_module::sub_module1::test2_fn2(); // error: function `test2_fn2` is private
}
