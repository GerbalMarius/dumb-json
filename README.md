### DJSON Library 
---
D stands for dumb, used to parse simple json represantations from a string literal in rust.
THIS LIBRARY SHOULD BE TREATED AS AN EXCERCISE IN TOKENIZING / PARSING AND SHOULD NOT  BE TAKEN SERIOUSLY (❁´◡`❁)
---
### Examples
---
1) Reading json from a string (valid utf-8):
```rust
use djson;

fn main() -> djson::Result<()>  {
    let input = "{\"age\": 0, \"name\": \"Marius\", \"emotion\":\"😊\"}";
    let result: JsonValue = djson::decode_json_str(input).unwrap();
    
    let age:     i32     = result.get("age").unwrap().int()?;
    let name:    &str    = result.get("name").unwrap().string()?;
    let emotion: &str    = result.get("emotion").unwrap().string()?;

    println!("age={}, name={}, emotion={}", age, name, emotion);
    Ok(())
}
```
2) Reading json list of objects from a file (valid utf-8, uses buffered io with reusable buffer for minimal footprint):
```rust
use djson;

const EXAMPLE_PATH: &str = "people.json";

fn main() -> djson::Result<()>  {
    let file_result = djson::decode_file(EXAMPLE_PATH).unwrap();

    let people: &[JsonValue] = file_result.get("people").unwrap().array()?;

    for person in people {
        let age = person.get("age").unwrap().int()?;
        let name = person.get("name").unwrap().string()?;
        let emotion = person.get("emotion").unwrap().string()?;

        println!("age={}, name={}, emotion={}", age, name, emotion);
    }

    Ok(())
}
```