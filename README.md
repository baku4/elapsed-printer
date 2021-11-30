# elapsed-printer
Very simple macro for printing time elapsed to execute a function.

## Feature
`elapsed-printer` is crate holding just *one* macro, `print_elapsed` using Rust standard library *`std::time`* to check elapsed during function(also method) execution.

## Attributes
`print_elapsed` can have three types of attributes. Using attributes is optional and, if not specified, uses the default attributes. The order of attributes and the use of quotes do not matter.
### (1) Stream to print time
1. `stdout` - Print output to standard output stream.
2. `stderr` - Print output to standard error stream.
3. `both` - Print output to both standard output and error stream.
* `Default`: `stdout`
### (2) Unit of time
1. `auto` - Print output in the form defined in *`Debug`* trait in *`std::time::Duration`* structure.
2. `s` - Print output in units of second.
3. `ms` - Print output in units of millisecond.
4. `us` - Print output in units of microsecond.
5. `ns` - Print output in units of nanosecond.
* `Default`: `auto`
### (3) Features list
* \[*features_list*\]
  * If any of the features in list are activated, print output.
  * If empty, print output always.
* `Default`: *not specified (=print always)*

## Example
### Use Cases
```rust
use elapsed_printer::print_elapsed;

#[print_elapsed]
fn func_to_print_elapsed_default() {}

#[print_elapsed(stdout, auto)]
// Same as default
// Print always regardless of feature activation.
fn func_to_print_elapsed_same_as_default() {}

#[print_elapsed(ms, "stdout")]
// Attribute order does not matter.
// Use of quotes does not matter.
fn func_to_print_elapsed_same_with_ms() {}

#[print_elapsed("ms", stderr, [feature_1])]
// Print when using `feature_1`
fn func_to_print_elapsed_when_using_feature_1() {}

#[print_elapsed([feature_1, feature_2], ns, stderr)]
// Print when using `feature_1` or `feature_2`
fn func_to_print_elapsed_when_using_feature_1_or_feature_2() {}

struct MyStruct;
impl MyStruct {
    #[print_elapsed]
    // Can be applied to method
    pub fn method(&self) {}
}
```
### Sample code and output
Code
```rust
use elapsed_printer::print_elapsed;
use std::time::Duration;
use std::thread;

#[print_elapsed]
fn function_name_1() {
    thread::sleep(Duration::from_millis(10));
}
#[print_elapsed(stdout, ns)]
fn function_name_2() {
    //
}
#[print_elapsed(stdout, us)]
fn function_name_3() {
    function_name_1()
}

fn main() {
    function_name_1();
    function_name_2();
    function_name_3();
    function_name_1();
}
```
Output
```ignore
function_name_1, 12.527014ms
function_name_2, 32ns
function_name_1, 10.070776ms
function_name_3, 10097us
```