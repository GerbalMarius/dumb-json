use djson;


const EXAMPLE_PATH: &str = "people.json";


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

    let people = file_result["people"].array()?;

    for person in people {
        let age = person["age"].unsigned_int()?;
        let name = person["name"].string()?;
        let emotion = person["emotion"].string()?;

        println!("age={}, name={}, emotion={}", age, name, emotion);
    }

    Ok(())
}