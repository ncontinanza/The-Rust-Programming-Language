fn main() {
    another_function(3, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let z = plus_one(11);
    println!("The value of z is: {}", z);
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // The following wouldn't work
    // x + 1;
    // x + 1 is an expression that evaluates to a value. x + 1; is a statement, which doesn't
    // evaluate to a value. Therefore, nothing is returned, which contradicts the function definition
    // and results in an error.
}
