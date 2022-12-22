fn branches() {
    let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    // inline If
    let foo = if number < 3 {5} else {6};
    println!("foo: {foo}");
}

fn loops() { 
    // while true
    // loop {println!("69");}
    
    let mut counter = 0;
    'counting: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {break;}
            if counter == 2 {break 'counting;}
            remaining -= 1;
        }
        counter += 1;
    }
}

fn for_loop() {
    let foo = [1,2,3,4,5,6,7];
    for elem in foo {
        println!("elem = {elem}");
    }

    for i in (1..4).rev() {
        println!("{i}");
    }
}

fn main() {
    // branches();
    // loops();
    for_loop();
}
