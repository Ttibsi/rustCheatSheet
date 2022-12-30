// Function using generics
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    return largest;
}

// Struct using generics
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// generic struct methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

fn main() {
    let num_list = vec![4, 18, 12, 69, 420, 34];
    let char_list = vec!['a', 'g', 'y', 'w', 'p', 'c', 't'];

    println!("Largest number is: {}", largest(&num_list));
    println!("Largest char is: {}", largest(&char_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);
    println!("int x = {}", integer.x());
}
