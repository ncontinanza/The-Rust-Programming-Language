fn main() {
    let mut counter = 0;

    // result will take the value from the loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            // The expression after break is returned and asigned to result
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
