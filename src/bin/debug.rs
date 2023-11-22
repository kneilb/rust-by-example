// 1.2.1. Debug

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

    // Debug print
    println!("{:?}", peter);

    // We can also manually implement fmt::Display to control the display
}
