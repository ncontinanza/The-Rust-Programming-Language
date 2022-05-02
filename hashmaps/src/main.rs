use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Such like vectors, hash maps store their values in the heap, so they have to be
    // homogeneous: all of the keys must have the same type, and all of the values must have
    // the same type as well.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // If "blue" and "yellow" would have been assigned to variables, say s1 and s2, the hash map would
    // now take ownership of them.

    // Building a hash map with collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Accessing values in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // score is Some(&10), because get returns an Option<&V>

    // We can iterate over each key/value pair:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    } // This prints each pair IN AN ARBITRARY ORDER

    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Hash maps do not allow repeated keys.
    // The value associated to "Blue" is now 25, and 10 is discarded.
    println!("{:?}", scores);

    // Only inserting a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // Associates 50 to the key "Yellow"
    scores.entry(String::from("Blue")).or_insert(50);   // This does not change the map, because "Blue" already has an associated value.
    // The return value of entry() is an enum Entry, that represents a value that might or might not exist.
    // The or_insert() method on Entry returns a mutable reference to the value for the corresponding
    // Entry key if it exists, and if not, inserts the parameter as the new value for this key and
    // returns a mutable reference to the new value.
    println!("{:?}", scores); // -> {"Yellow": 50, "Blue": 10}

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { //split_whitespace() iterates over sub-slices separated by whitespace
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereferencing with * allows us to change the value in the hash map.
    } // The mutable reference to count goes out of scope here, and everything remains safe :D

    println!("{:?}", map); // -> {"world": 2, "hello": 1, "wonderful": 1}
}
