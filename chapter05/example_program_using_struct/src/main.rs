fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of rectangle is {} square pixels.",
             area(width1, height1)
    );

    // Refactoring with Tuples
    let rect1 = (30, 50);
    println!("The area of rectangle is {} square pixels.",
             area2(rect1)
    );

    // Adding Useful Functionality with Derived Traits
    let rect2 = Rectangle{width: 30, height: 50};
    println!("rect2 is {:?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}