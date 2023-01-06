fn conditional_structures() {
    let fave_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = fave_colour {
        println!("Using color: {colour} as background");
    } else if is_tuesday {
        println!("Tuesday is green");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Using blue");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn enumerate_tuple() {
    let v = vec!['a', 'b', 'c'];

    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }
}

fn multi_pattern() {
    let x = 7;

    match x {
        1 | 2 => println!("One or two"),
        3..=10 => println!("three through ten"),
        _ => println!("anything else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn match_struct_elems() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // create vars a and b this way
    let Point { x, y } = p; // Store values in x and y vars

    assert_eq!(0, a);
    assert_eq!(b, 7);
    assert_eq!(0, x);
    assert_eq!(y, 7);

    match p {
        Point { x, y: 0 } => println!("Match on x at {x}"),
        Point { x: 0, y } => println!("Match on y at {y}"),
        Point { x, y } => println!("match on neither ({x}, {y})"),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ColorToo(Color),
}

fn match_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move {x}, {y}"),
        Message::Write(foo) => println!("Write: {foo}"),
        Message::ChangeColor(r, g, b) => println!("Color: {r}, {g}, {b}"),
        // Nested destructuring and matching
        Message::ColorToo(Color::Rgb(r, g, b)) => println!("RGB: {r}, {g}, {b}"),
        Message::ColorToo(Color::Hsv(h, s, v)) => println!("HSV: {h}, {s}, {v}"),
    }
}

fn ignore_parts() {
    let p = Point { x: 0, y: 7 };
    let nums = (1, 2, 3, 4, 5, 6, 7, 8, 9);

    match p {
        // the .. here ignores all other attributes, no matter how many there are
        Point { x, .. } => println!("x is {}", x),
    }

    match nums {
        (first, .., last) => println!("numbers: {first}, {last}"),
    }
}

fn match_guards() {
    let num = Some(4);
    let y = 4;

    match num {
        // Some(x) if x % 2 == 0 => println!("even"),
        Some(n) if n == y => println!("Matched n = {n}"),
        Some(_x) => println!("odd"),
        None => (),
    }
}

fn main() {
    // conditional_structures()
    // while_let()

    // enumerate_tuple()

    // Multiple assignment does work too
    let (a, b) = (1, 2);
    println!("{}", a);

    if let Some(x) = Some(a) {
        println!("{}", x);
    }

    // multi_pattern()
    // match_struct_elems()
    // match_enum()
    match_guards()
}
