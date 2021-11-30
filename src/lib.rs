use elapsed_printer_macros::print_elapsed;
use std::time::{Duration};

#[cfg(test)]
mod tests {
    use super::*;

    use std::{marker::PhantomData, thread};

    struct TestStruct<T> {
        test_field: u64,
        pd: PhantomData<T>,
    }

    impl<T> TestStruct<T> {
        #[print_elapsed([test_feature, a])]
        fn new() -> Self {
            Self {
                test_field: 10,
                pd: PhantomData,
            }
        }
        #[print_elapsed([test_feature, aa])]
        pub fn sleep_millis_method(&self, millis: u64) -> u64 {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
            10
        }
        #[print_elapsed]
        fn sleep_millis_function(millis: u64) {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
        }
    }

    #[print_elapsed]
    fn sleep_millis(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }

    #[test]
    fn print_time() {
        // Function
        #[cfg(test_feature)]
        println!("TEST TEST FEATUREs1");
        #[cfg(any(feature = "test_feature", feature = "a"))]
        println!("TEST TEST FEATUREs2");

        sleep_millis(10);

        // Method in Struct
        let test_struct: TestStruct<usize> = TestStruct::new();
        test_struct.sleep_millis_method(10);

        // Function in Struct
        TestStruct::<usize>::sleep_millis_function(10);
    }
}