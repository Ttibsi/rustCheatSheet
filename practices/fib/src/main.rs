fn main() {
    let mut toggle = false;
    let mut num1 = 0;
    let mut num2 = 1;

    println!("{num1}");

    for _ in 1..13 {
        if toggle {
            num1 += num2;
            println!("{num1}");
            toggle = false;
        } else {
            num2 += num1;
            println!("{num2}");
            toggle = true;
        }
    }
}
