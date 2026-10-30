#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50
    };

    println!("The area of the rectangle is {} square pixels.",
     area(&rect));
    
    println!("Rectangle: {rect:?}");
    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32{
    rect.height * rect.width
}
