struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    } // here x is deallocated, but r is still valid for the outer scope.
    // This would fail, because r is referencing invalid memory:
    // println!("r: {}", r);}

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // The following wouldn't work:
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // Novel doesn't go out of scope until after the ImportantExcerpt goes out of scope
    // So the reference in the ImportantExcerpt instance is invalid.

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // Here we make explicit that the lifetime
    // of the parameters and the lifetime of the returned reference are related such that the
    // returned reference will be valid as long as both the parameters are.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
