#![allow(dead_code)]
#![allow(unused_imports)]
use elapsed_printer::print_elapsed;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{marker::PhantomData, thread};

    struct TestStruct<T> {
        any_number: u64,
        pd: PhantomData<T>,
    }

    impl<T> TestStruct<T> {
        #[print_elapsed]
        fn new() -> Self {
            Self {
                any_number: 10,
                pd: PhantomData,
            }
        }
        #[print_elapsed]
        fn slow_new(millis: u64) -> Self {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
            
            Self {
                any_number: 10,
                pd: PhantomData,
            }
        }
        #[print_elapsed]
        pub fn sleep_millis_pub_method(&self, millis: u64) {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
        }
        #[print_elapsed]
        fn sleep_millis_return_one_hundred(millis: u64) -> u32 {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
            100
        }
    }

    #[print_elapsed]
    fn sleep_millis(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }

    #[print_elapsed("s")]
    fn print_s(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("ms")]
    fn print_ms(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("us")]
    fn print_us(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("ns")]
    fn print_ns(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("stdout", s)]
    fn print_s_to_stdout(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("stderr", ms)]
    fn print_ms_to_stderr(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed("both", us)]
    fn print_us_to_both(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed([feature_1], stdout, auto)]
    fn print_when_feature_1(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed(stdout, [feature_1, "feature_2"], auto)]
    fn print_when_feature_1_or_2(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }
    #[print_elapsed(auto, stdout, ["feature_1", feature_2, "feature_3"])]
    fn print_when_feature_1_or_2_or_3(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }

    #[test]
    fn print_macros() {
        // Function
        sleep_millis(10);
        // Method in Struct
        let _: TestStruct<usize> = TestStruct::new();
        let test_struct: TestStruct<usize> = TestStruct::slow_new(15);
        test_struct.sleep_millis_pub_method(20);
        // Function in Struct
        let value = TestStruct::<usize>::sleep_millis_return_one_hundred(10);
        assert_eq!(value, 100);

        // Test with attributes
        print_s(1000);
        print_ms(10);
        print_us(10);
        print_ns(10);
        print_s_to_stdout(1000);
        print_ms_to_stderr(10);
        print_us_to_both(10);
        print_when_feature_1(10);
        print_when_feature_1_or_2(10);
        print_when_feature_1_or_2_or_3(10);
    }
}