use std::collections::HashMap;

fn get_most_common(v: Vec<i32>) -> i32 {
    let mut counter = HashMap::new();

    for i in v {
        let count = counter.entry(i).or_insert(0);
        *count += 1;
    }

// let key_with_max_value = a_hashmap.iter().max_by_key(|entry | entry.1).unwrap();
    let highest = counter.iter().max_by_key(|entry | entry.1).unwrap();
    return *highest.0;
}

fn main() {
    let mut v = vec![2,3,12, 69, 420, 32, 96,2, 16, 12, 1, 42];
    v.sort();
    let len = v.len();

    println!("{:?}", v);
    println!("Middle value: {}", v[len/2]);

    let mcm = get_most_common(v);
    println!("Most commom value: {}", mcm);
}
