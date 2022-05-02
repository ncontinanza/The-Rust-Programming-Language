use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");
    // if File::open succeeds, f will be an instance of Ok that contains a file handle.
    // if File::open fails, f will be an instance of Err that contains info about the kind of error that happened.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If the file was not found, we try to create it instead of panicking.
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // Instead of so many match, we could have also used unwrap_or_else
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // This is also possible, but it panics upon failure:
    let f = File::open("hello.txt").unwrap();

    // A way to choose the panic! error message is to use expect() instead of unwrap():
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// This function propagates an error
fn read_username_from_file() -> Result<String, io::Error> {
    // The return type contains an error.
    // If the function succeeds, the caller will receive an OK that holds a String.
    // If the function fails, the caller will receive an Err that holds an instance of io::Error.
    let f = File::open("hello.txt"); // If an error happens here, its delegated to the caller.

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // This is why the return type of the function contains an error.
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A shortcut for propagating errors: ?
fn read_username_from_file2() -> Result<String, io::Error> {
    // The ? operator placed after a Result value works in almost the same way as the match expressions we used before.
    // The difference: the ? operator converts the error type received to the error type defined
    // in the return type of this function. It uses the from function defined in the From trait.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// An even shorter version of the same function:
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// ? operator can also be used when the function returns an Option:
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // If an error occurs, None contains the error.
    // If the function succeeds, Some contains the resulting value.
}


