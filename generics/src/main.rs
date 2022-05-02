fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We can eliminate the code duplication using a generic type parameter in a single function.
// We must be explicit in the fact that of all the possible types that we could receive, T values must be able to be ordered.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// We can use generics in struct definitions:
struct Point<T, U> {
    x: T,
    y: U,
}

// We can declare methods for any generic types T and U:
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// And also declare methods for a particular type:
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
} // distance_from_origin() only exists for instances of Point<f32, f32>

// We can use generics in enum definitions:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
