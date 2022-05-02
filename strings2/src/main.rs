fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // strings are UTF-8 encoded!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Append to a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // push_str takes a string slice, so we can still use s2 in this scope.
    println!("s2 is {}", s2);

    // To add a single character, we use push
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The + operator uses the add method: fn add(self, s: &str) -> String
    // (actually it uses generics, but I substituted them for understanding)

    // Let's concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // This seems uncomfortable when scaling. We can use the format! macro, which is a little bit simpler:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into strings
    // The following will throw an error:
    let s1 = String::from("hello");
    let h = s1[0];

    // Iterating over strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    } // This prints the 6 characters that form the string.

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    } // This prints the 18 bytes that make up the string.

}
