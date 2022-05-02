fn main() {
    panic!("crash and burn");
    // After panic, rust walks up the stack and cleans up the data from each function.

    let v = vec![1, 2, 3];
    v[99]; // This panics, but not in our code.

}
