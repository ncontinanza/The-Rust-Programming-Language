fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // The following are equal
    let slice = &s[0..2];
    let slice = &s[..2];

    // And also are these
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // Both values can also be dropped
    let slice = &s[0..len];
    let slice = &s[..];

}
