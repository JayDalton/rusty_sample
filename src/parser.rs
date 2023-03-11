use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

pub fn untyped_example(data: &str) -> Result<()> {

    let v: Value = serde_json::from_str(data)?;

    println!("JSON Object: {:?}", v);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn typed_example(data: &str) -> Result<()> {

    let p: Person = serde_json::from_str(data)?;

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    println!("Please call {:?}", p);

    Ok(())
}
