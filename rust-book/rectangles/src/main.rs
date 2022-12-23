#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function that's not a method - usually for constructors
    fn square(size: u32) -> Self {
        Self{width: size, height: size}
    }
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}

fn area_with_tuple(dimensions:(u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn main() {
    let rect1 = (12, 4);
    println!("The area of the first rectangle is {}", area_with_tuple(rect1));

    let rect2 = Rectangle{ width:14, height: 5};
    println!("The area of the second rectangle is {}", area_with_struct(&rect2));
    println!("The area of the second rectangle as a method is {}", rect2.area()); 

    println!("Rectangle 2 is: {:#?}", rect2);
    dbg!(&rect2);

    let rect3 = Rectangle{ width:8, height: 4};
    println!("Can rect2 fit rect3 inside it? {}", rect2.can_hold(&rect3));
    println!("Can rect3 fit rect2 inside it? {}", rect3.can_hold(&rect2));

    let sq = Rectangle::square(4);
    println!("{:#?}", sq);


}
