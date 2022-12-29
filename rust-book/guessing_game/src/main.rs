use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub struct Guess { value: i32, }
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100, got {}", value);
        }

        Guess{value}
    }

    // Getter, needed because value attr is private
    pub fn value(&self) -> i32 { self.value }
}

fn new_main() {
    println!("Guess the number!");
    println!("Input your guess");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let val:i32 = guess.trim().parse().unwrap();

        let guess = Guess::new(val);
        println!("Your guess: {:?}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}


// fn old_main() {
//     println!("Guess the number!");
//     println!("Input your guess");
//
//     let secret_number = rand::thread_rng().gen_range(1..=10);
//
//     loop {
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("Failed to read");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("Your guess: {guess}");
//
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big"),
//             Ordering::Equal => {
//                 println!("You win");
//                 break;
//             }
//         }
//     }
// }

fn main() { new_main(); }
