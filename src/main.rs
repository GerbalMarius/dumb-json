use djson;


const EXAMPLE_PATH: &str = "people.json";

#[derive(djson::Decode)]
struct Person {
    age: u32,
    name: String,
    emotion: String
}

fn main() -> djson::Result<()>  {
    let input = "{\"age\": 0, \"name\": \"Marius\", \"emotion\":\"😊\"}";
    let result = djson::decode_json_str(input).unwrap();
    djson::pp_print_stdout(&result);
    
    let age = result["age"].unsigned_int()?;
    let name = result["name"].string()?;
    let emotion = result["emotion"].string()?;

    println!("age={}, name={}, emotion={}", age, name, emotion);
    println!();

    println!("Reading from a file: ");

    let file_result = djson::decode_file(EXAMPLE_PATH).unwrap();

    let people = djson::decode_json::<Vec<Person>>(&file_result["people"])?;

    for p in &people {
        println!("age={}, name={}, emotion={}", p.age, p.name, p.emotion);
    }

    Ok(())
}