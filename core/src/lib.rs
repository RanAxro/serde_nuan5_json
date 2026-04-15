// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


mod r#struct;
mod ext_type;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// #[derive(Serialize, Deserialize)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

#[test]
fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // let p: Person = serde_json::from_str(data)?;
    //
    // println!("Please call {} at the number {}", p.name, p.phones[0]);

    let t: Value = serde_json::from_str(data)?;
    println!("{}", t);

    Ok(())
}

