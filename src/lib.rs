use elapsed_printer_macros::test_macro;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use super::*;

    use std::{marker::PhantomData, thread};

    struct TestStruct<T> {
        test_field: u64,
        pd: PhantomData<T>,
    }

    impl<T> TestStruct<T> {
        #[test_macro(stdout, "t")]
        fn new() -> Self {
            Self {
                test_field: 10,
                pd: PhantomData,
            }
        }
        #[test_macro]
        pub fn sleep_millis_method(&self, millis: u64) -> u64 {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
            10
        }
        #[test_macro]
        fn sleep_millis_function(millis: u64) {
            let millis = Duration::from_millis(millis);
            thread::sleep(millis);
        }
    }

    #[test_macro]
    fn sleep_millis(millis: u64) {
        let millis = Duration::from_millis(millis);
        thread::sleep(millis);
    }

    #[test]
    fn print_time() {
        let start = Instant::now();

        // Function
        sleep_millis(10);
        let duration = start.elapsed();
        println!("Function: {:?}", duration);

        // Method in Struct
        let test_struct: TestStruct<usize> = TestStruct::new();
        test_struct.sleep_millis_method(10);
        let duration = start.elapsed();
        println!("Method in Struct: {:?}", duration);

        // Function in Struct
        TestStruct::<usize>::sleep_millis_function(10);
        let duration = start.elapsed();
        println!("Function in Struct: {:?}", duration);
    }
}