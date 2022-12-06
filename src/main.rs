mod functions;

#[allow(unused_imports)]
use functions::tutorial_functions::{
    array_test, generate_random_ints, mut_tuple, take_int_input, take_input, add_numbers, sub_numbers,
};

fn main() {
    add_numbers(5, 6);
    println!("The result is: {}", sub_numbers(5, 6));
}