use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // thread_rng -> random number generator local to the current thread
    // gen_range -> takes a range and generates a random number in the range.
    // it's inclusive on the lower bound but exclusive on the upper bound
    // 1..=100 to make it inclusive on the upper bound as well
    let secret_number = rand::thread_rng().gen_range(1..101);

    // For testing and debugging purposes:
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // mut -> Mutable
        // ::new -> new is an associated function of the String type, like a static method
        // Note that guess' type is being inferred
        let mut guess = String::new();

        // Without use -> std::io::stdin
        // io::stdin returns an instance of std::io::Stdin
        io::stdin()
            // & -> this argument is a reference. Like variables, references are immutable by default.
            .read_line(&mut guess)
            // expect is used to handle possible errors
            .expect("Failed to read line");

        // We define a new immutable variable guess, shadowing the previous value.
        // This feature is often used in situations where we want to convert a value from one type
        // to another type.
        // trim() will eliminate whitespaces such as the \n at the end of this input.
        // parse() parses a string into the kind of number stated after the colon (:)
        // parse() returns a Result type, so we can use match to handle possible errors.
        let guess: u32 = match guess.trim().parse() {
            // If Ok, we return the number parsed and it's binded to the guess variable
            Ok(num) => num,
            // If Err, we jump to the next iteration of the loop. _ is a catchall value.
            Err(_) => continue,
        };

        // Placeholders, same as in C
        println!("You guessed: {}", guess);

        // Ordering is an enum: Less, Greater and Equal are its values.
        // cmp method compares two values and can be called on anything that can be compared
        // cmp returns a variant of the Ordering enum
        // match is used to decide what to do next based on cmp's return value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
