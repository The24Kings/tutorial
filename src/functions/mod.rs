pub mod tutorial_functions {
    use std::io;
    use rand::Rng;

    #[allow(dead_code)]
    pub fn take_int_input() {
        let mut input = String::new();
        println!("Enter a number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let int_input = input.trim().parse::<i32>().unwrap();

        println!("You entered: {}", int_input + 2);
    }

    #[allow(dead_code)]
    pub fn take_input() {
        let mut input = String::new();

        println!("Enter a number: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You entered: {}", input);
    }

    #[allow(dead_code)]
    pub fn mut_tuple() {
        let mut tup = (1, true, 's', 3.14, "Hello World");

        println!("Tuple: {:?}", tup);

        tup.0 = 10;

        println!("Tuple: {:?}", tup);
    }

    #[allow(dead_code)]
    pub fn generate_random_ints() {
        const LOWER_LIMIT: i8 = 1;
        const UPPER_LIMIT: i8 = 5;

        let max: i8 = 10;

        for _i in 1..max {
            let rand_int = rand::thread_rng().gen_range(LOWER_LIMIT..UPPER_LIMIT);
            println!("Random number: {}", rand_int);
        }
    }

    #[allow(dead_code)]
    pub fn array_test() {
        let arr = [1, 2, 3, 4, 5];

        println!("Array: {:?}", arr);

        let arr2 = [3; 5];

        println!("Array: {:?}", arr2);
    }

    #[allow(dead_code)]
    pub fn add_numbers(a: i32, b: i32) {
        println!("Sum is: {}", a + b);
    }

    pub fn sub_numbers(a: i32, b: i32) -> i32 {
        return a - b;
    }
}