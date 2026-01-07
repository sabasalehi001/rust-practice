use serde_json::{Value, Map};
use clap::{Arg, App};
use std::fs;
use std::process;

fn main() {
    let matches = App::new("Data Inspector")
        .version("0.1.0")
        .author("Your Name")
        .about("Validates JSON data against a schema.")
        .arg(
            Arg::new("schema")
                .short('s')
                .long("schema")
                .value_name("FILE")
                .help("Sets the schema file to use")
                .required(true),
        )
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .value_name("FILE")
                .help("Sets the data file to validate")
                .required(true),
        )
        .get_matches();

    let schema_file = matches.value_of("schema").unwrap();
    let data_file = matches.value_of("data").unwrap();

    let schema_content = fs::read_to_string(schema_file)
        .unwrap_or_else(|err| {
            eprintln!("Error reading schema file: {}", err);
            process::exit(1);
        });

    let data_content = fs::read_to_string(data_file)
        .unwrap_or_else(|err| {
            eprintln!("Error reading data file: {}", err);
            process::exit(1);
        });

    let schema: Value = serde_json::from_str(&schema_content)
        .unwrap_or_else(|err| {
            eprintln!("Error parsing schema: {}", err);
            process::exit(1);
        });

    let data: Value = serde_json::from_str(&data_content)
        .unwrap_or_else(|err| {
            eprintln!("Error parsing data: {}", err);
            process::exit(1);
        });

    if validate(&data, &schema) {
        println!("Data is valid.");
    } else {
        println!("Data is invalid.");
        process::exit(1);
    }
}

fn validate(data: &Value, schema: &Value) -> bool {
    match (data, schema) {
        (Value::Object(data_obj), Value::Object(schema_obj)) => {
            for (key, schema_value) in schema_obj {
                if let Some(data_value) = data_obj.get(key) {
                    if !validate(data_value, schema_value) {
                        println!("Validation failed for key: {}", key);
                        return false;
                    }
                } else {
                    println!("Key {} is missing in data.", key);
                    return false;
                }
            }
            true
        }
        (Value::Array(data_array), Value::Array(schema_array)) => {
            if let Some(schema_element) = schema_array.get(0) { // Assuming schema has only one example item
                for data_element in data_array {
                    if !validate(data_element, schema_element) {
                        return false;
                    }
                }
                true
            } else {
                true // Empty schema array means any array is valid
            }
        }
        (Value::String(_), Value::String(_)) => true, // String type check
        (Value::Number(_), Value::Number(_)) => true, // Number type check
        (Value::Bool(_), Value::Bool(_)) => true, // Boolean type check
        (Value::Null, Value::Null) => true, // Null type check
        (_, Value::String(expected_type)) => {
            match expected_type.as_str() {
                "string" => data.is_string(),
                "number" => data.is_number(),
                "boolean" => data.is_boolean(),
                "null" => data.is_null(),
                "object" => data.is_object(),
                "array" => data.is_array(),
                _ => {
                    eprintln!("Unsupported schema type: {}", expected_type);
                    false
                }
            }
        }
        _ => {
            println!("Type mismatch: Data is {:?}, Schema is {:?}", data, schema);
            false
        }
    }
}