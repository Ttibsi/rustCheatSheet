use d::cmp::Ordering;
use d::io;
use nd::Rng;

fn main() {
    println!("Guess the number!");
    println!("Input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
