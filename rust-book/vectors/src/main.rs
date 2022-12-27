fn main() {
    let v1 : Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3]; // Using a macro to define a vector

    v2.push(4);
    println!("{:?}", v2);

    // let third: &i32 = &v2[3];
    // println!("{}", third);

    let third: Option<&i32> = v2.get(3);
    match third {
        Some(third) => println!("Third is {}", third),
        None => println!("Not found"),
    }

    for i in &v2 {println!("{i}");}
    for i in &mut v2 {println!("{}", *i*2);}
}
