fn main() {
    // Different ways of creating a vector
    let empty_vec: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Updating a vector
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    } // Here v is dropped, and so are its elements

    // Reading from a vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element."),
    }

    // We can use &v[2] or v.get(2). The first gives us a reference, and the second an Option<T>.
    // Both are valid and idiomatic, and choosing one or the other depends on the expected behaviour.
    // Particularly, what we want the program to do when we try to use an index value out of range.

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // This panics.
    let does_not_exist = v.get(100); // This will return a None.

    // Attempting to add an element to a vector while holding a reference to an item will also fail:
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Immutable borrow occurs here

    v.push(6); // Mutable borrow occurs here, and the program fails.

    println!("The first element is: {}", first); // Immutable borrow is used here

    // Iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Rust needs to know what types will be in the vector at compile time, so we can use enums
    // To store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
