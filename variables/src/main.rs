fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
//  ------------------------------------------------------------------------------------------------
    // Shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);
//  ------------------------------------------------------------------------------------------------
    // Shadowing also lets us mutate a variable's type
    let _spaces = "     ";
    let _spaces = _spaces.len();
}
