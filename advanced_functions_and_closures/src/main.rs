fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // fn is a function pointer
    f(arg) + f(arg)
}

// We can also return a closure, using a Box
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5); // we send the function add_one as an argument. 

    println!("The answer is: {}", answer);

    // Another example is in the use of map()
    let list_of_numbers = vec![1, 2, 3];
    let list_of_numbers: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}
