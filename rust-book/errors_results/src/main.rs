use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_res = File::open("t.txt");

    let my_file = match file_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("t.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        },
    };
}
