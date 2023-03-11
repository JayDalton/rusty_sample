use std::fs;

pub mod parser;

fn main() {
    println!("Hello, world!");

    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    match parser::typed_example(data) {
        Ok(()) => (),
        Err(error) => print!("{}", error),
    }

    let file_path = "data/test.json".to_owned();
    let content = fs::read_to_string(file_path)
        .expect("File does not exist.");

    let result = parser::untyped_example(&content);

    match result {
        Ok(()) => (),
        Err(error) => print!("{}", error),
    }
}
