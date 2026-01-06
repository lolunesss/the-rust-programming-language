fn main() {
    function_with_no_args();
    function_with_one_arg(42);
    function_with_one_arg_and_return(10);
}

fn function_with_no_args() {
    println!("Function with no args");
}

fn function_with_one_arg(number: i32) {
    println!("Function with one number: {number} arg");
}

fn function_with_one_arg_and_return(number: i32) -> i32 {
    number + 10
}
