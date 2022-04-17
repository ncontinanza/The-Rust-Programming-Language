fn main() {
    // This is immutable
    let s = "Hello, world!";
    println!("{}", s);

    // String is mutable, if desired
    let mut str = String::from("Hello");
    str.push_str(", world!");
    println!("{}", str);

    let s1 = String::from("hello");

    // Move
    let s2 = s1;
    // s1 is now invalidated, and it's data (pointer, length and capacity) was moved to s2.

    // The following wouldn't work:
    // println!("{}, world!", s1);

    // If, instead, we used s1.clone, the last line would compile, because the heap data gets copied.

    let x = 5;
    let y = x;

    // Types such as integers that hava a known size at compile time are stored entirely on the stack.
    // So, the following is valid:
    println!("x = {}, y = {}", x, y);
}
