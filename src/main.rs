pub mod parser;

fn main() {
    println!("Hello, world!");
    // parser::typed_example();

    let result = parser::typed_example();

    match result {
        Ok(()) => (),
        Err(error) => print!("{}", error),
    }
}
