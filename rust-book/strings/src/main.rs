fn do_thing(s: &mut String) {
    s.push_str(" there");
}

fn main() {
    let mut s = String::from("hi");
    do_thing(&mut s);
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world!");
    let s3 = format!("{s1} {s2}");
    println!("{s3}");

    // Taken straight from the book, this demonstrates characters that are longer
    // than a single byte - S4 holds the first two letters of hello which are
    // two bytes long each in utf-8
    let hello = "Здравствуйте";
    let s4 = &hello[0..4];

    // Print each character in string, which may not be appropriate for every
    // language
    for c in hello.chars() {
        println!("{c}");
    }

    // Print each byte in string
    for b in hello.bytes() {
        println!("{b}");
    }

    // If you need grapheme clusters, check out crates
}
