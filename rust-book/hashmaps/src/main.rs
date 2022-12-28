use std::collections::HashMap;

fn use_as_counter() {
    let text = "The quick brown fox jumps over the quick brown fox";
    let mut counter = HashMap::new();

    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("blue"), 50);

    let team = String::from("Red");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{score}");

    scores.insert(String::from("Red"), 25); // Change value

    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    println!("{:?}", scores.entry(String::from("blue")));
    scores.entry(String::from("blue")).or_insert(4);
    println!("{:?}", scores);

    use_as_counter();
}
