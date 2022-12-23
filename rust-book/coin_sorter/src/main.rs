enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn get_value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let c = Coin::Quarter;
    println!("My coin is {}", get_value(c));
}
