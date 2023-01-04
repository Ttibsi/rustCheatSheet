// Implementing some stuff to mock out a GUI library.

// A trait function to draw the objects to the screen
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // A vec of any items that may or may not have the Draw trait
    pub components: Vec<Box<dyn Draw>> 
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Now for some actual objects to draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button at {0}x{1}: {2}", self.width, self.height, self.label);
    }
}
