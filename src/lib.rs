use serde::Deserialize;
use serde_json::{de::IoRead, StreamDeserializer, Value};
use std::collections::HashSet;
use std::error::Error as StdError;
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

pub fn stream_parse_and_filter_json_file(
    path: &str,
    filter_key: &str,
    filter_value: &str,
) -> Result<(), Box<dyn StdError>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let stream: StreamDeserializer<_, Item> = StreamDeserializer::new(IoRead::new(reader));

    println!("Starting streaming parsing and filtering...");
    for item_result in stream {
        match item_result {
            Ok(item) => {
                if filter_key == "name" && item.name == filter_value {
                    println!("Filtered item: {:#?}", item);
                }
            }
            Err(e) => eprintln!("Error parsing item: {}", e),
        }
    }
    println!("Streaming parsing and filtering finished.");
    Ok(())
}

pub fn compare_json_structures(
    file1_path: &str,
    file2_path: &str,
) -> Result<bool, Box<dyn StdError>> {
    let file1 = File::open(file1_path)?;
    let reader1 = BufReader::new(file1);
    let value1: Value = serde_json::from_reader(reader1)?;

    let file2 = File::open(file2_path)?;
    let reader2 = BufReader::new(file2);
    let value2: Value = serde_json::from_reader(reader2)?;

    Ok(compare_value_structures(&value1, &value2))
}

fn compare_value_structures(val1: &Value, val2: &Value) -> bool {
    match (val1, val2) {
        (Value::Object(map1), Value::Object(map2)) => {
            let keys1: HashSet<_> = map1.keys().collect();
            let keys2: HashSet<_> = map2.keys().collect();

            if keys1 != keys2 {
                return false;
            }

            for key in keys1 {
                if !compare_value_structures(&map1[key], &map2[key]) {
                    return false;
                }
            }
            true
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            if arr1.len() != arr2.len() {
                return false;
            }
            if arr1.is_empty() && arr2.is_empty() {
                return true;
            }
            if !arr1.is_empty() && !arr2.is_empty() {
                compare_value_structures(&arr1[0], &arr2[0])
            } else {
                true
            }
        }
        (Value::String(_), Value::String(_)) => true,
        (Value::Number(_), Value::Number(_)) => true,
        (Value::Bool(_), Value::Bool(_)) => true,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

pub fn generate_large_json_file(path: &str, num_items: usize) -> Result<(), Box<dyn StdError>> {
    let mut file = File::create(path)?;

    for i in 1..=num_items {
        let item = Item {
            id: i as u32,
            name: format!("item{}", i),
        };
        serde_json::to_writer(&mut file, &item)?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
