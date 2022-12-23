fn do_thing(s: &mut String) {
    s.push_str(" there");
}

fn main() {
    let mut s = String::from("hi");
    do_thing(&mut s);
    println!("{}", s);
}
