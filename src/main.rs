pub mod djson;

fn main()  {
    let input = "{\"age\": 0, \"name\": \"Marius\", \"emotion\":\"😊\"}";
    let result = djson::decode_json_str(input).unwrap();
    
    let age = result.get("age").unwrap().number();
    let name = result.get("name").unwrap().string();
    let emotion = result.get("emotion").unwrap().string();

    println!("age={}, name={}, emotion={}", age, name, emotion);
}