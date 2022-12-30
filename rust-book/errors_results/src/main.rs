use std::fs;
use std::io::ErrorKind;

fn main() {
    let file_res = File::open("t.txt");

    // let my_file = match file_res {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("t.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening file: {:?}", other_error)
    //         }
    //     },
    // };

    // This line does the same as above
    let my_file = File::open("t.txt").expect("File not found");
    let file_content = fs::read_to_string("t.txt");
    println!("{}", file_content);
}

// The below function does a similar thing to above, reading the contents of
// a file into the `user` string.
// The ? operator acts similarly to a match statement (ish), acting as an early
// return of a value or error. This can't be used in main() because main() can't
// return anything
use std::fs::File;
use std::io::{self, Read};

fn get_username_from_file() -> Result<String, io::Error> {
    let mut user = String::new();
    File::open("t.txt")?.read_to_string(&mut user);
    Ok(user)
    // fs::read_to_string("t.txt") // This one line is the same as the above 3
}
