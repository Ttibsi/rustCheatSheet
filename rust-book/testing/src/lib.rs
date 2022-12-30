#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a:i32) -> i32 {
    return a + 2;
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name); //This passes
    // return String::from("Hello"); // This intentionally fails
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{ width:8, height: 7,};
        let smaller = Rectangle{ width:5, height: 1,};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle{ width:8, height: 7,};
        let smaller = Rectangle{ width:5, height: 1,};
        assert!(!smaller.can_hold(&larger))
    }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    //     assert_ne!(add_two(2), 5);
    // }
    //
    #[test]
    fn greeting_contains_name() {
        let ret = greeting("lili");
        assert!(
            ret.contains("lili"),
            "Greeting did not contain name. Value was: {}", ret
        );
    }
}

// You can split the tests up into modules like this and run 
// `cargo test more_tests` to specify which module of tests to run. You can also
// use this method to specify individual tests to run
#[cfg(test)]
mod more_tests {
    use super::*;

    #[test]
    #[should_panic(expected="less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Run an ignored test with `cargo test -- --ignored` or to run all tests
    // do `cargo test -- --include-ignored
    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 { return Ok(()); }
        else { return Err(String::from("2 + 2 does not equal 4")); }
    }
}
