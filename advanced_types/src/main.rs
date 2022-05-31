type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn bar() -> ! { // ! means that this fn returns never.
    panic!();
}

