use inheritance_via_traits::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl inheritance_via_traits::Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a selectbox at {0}x{1}", self.width, self.height);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("Hello world"),
            }),
            Box::new(Button {
                width: 20,
                height: 15,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 20,
                height: 15,
                options: vec![
                    String::from("One"),
                    String::from("Two"),
                    String::from("Three")
                ]
            }),
        ]
    };

    screen.run();
}
