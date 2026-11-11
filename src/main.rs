use djson;


const EXAMPLE_PATH: &str = "people.json";


fn main() -> djson::Result<()>  {
    let input = "{\"age\": 0, \"name\": \"Marius\", \"emotion\":\"😊\"}";
    let result = djson::decode_json_str(input).unwrap();
    djson::pp_print_stdout(&result);
    
    let age = result.get("age").unwrap().unsigned_int()?;
    let name = result.get("name").unwrap().string()?;
    let emotion = result.get("emotion").unwrap().string()?;

    println!("age={}, name={}, emotion={}", age, name, emotion);
    println!();

    println!("Reading from a file: ");

    let file_result = djson::decode_file(EXAMPLE_PATH).unwrap();

    let people = file_result.get("people").unwrap().array()?;

    for person in people {
        let age = person.get("age").unwrap().unsigned_int()?;
        let name = person.get("name").unwrap().string()?;
        let emotion = person.get("emotion").unwrap().string()?;

        println!("age={}, name={}, emotion={}", age, name, emotion);
    }

    Ok(())
}